//! The admin plane: health, readiness, and metrics on a separate listener
//! (`SV4.3`, restating `O10.9`).
//!
//! Separate bind address, not separate routes: operational endpoints must be
//! exposable to an operations network without exposing the FHIR API's
//! clinical data to it. The main listener carries PHI; this one must never.
//!
//! Latency is a **histogram**, not a running total, so p99 is answerable
//! (`SV4.3`). The buckets are fixed at compile time and counted with atomics —
//! no locks on the request path, no metrics dependency in the supply chain.
//! The exposition is the Prometheus text format, which is a stable, trivial
//! contract worth more than a crate.

use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;

/// Upper bounds of the latency buckets, in seconds. Cumulative, Prometheus
/// style; an implicit `+Inf` bucket follows.
const BUCKETS: [f64; 12] = [
    0.001, 0.0025, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0,
];

/// Request latencies and outcomes for the whole FHIR listener.
///
/// One histogram overall rather than one per route: the route template is in
/// the tracing spans already, and a per-route family here would grow with the
/// resource-type count. The status-class counters answer "how much is
/// failing"; the histogram answers "how slow is it".
#[derive(Default)]
pub struct Metrics {
    /// Cumulative-style bucket counts; `bucket[i]` counts durations `<=
    /// BUCKETS[i]`. Stored non-cumulatively and summed at exposition, so the
    /// hot path is one `fetch_add`.
    buckets: [AtomicU64; 12],
    overflow: AtomicU64,
    count: AtomicU64,
    sum_micros: AtomicU64,
    /// Responses by status class: index 0 => 1xx … index 4 => 5xx.
    by_class: [AtomicU64; 5],
}

static METRICS: Metrics = Metrics {
    buckets: [
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
    ],
    overflow: AtomicU64::new(0),
    count: AtomicU64::new(0),
    sum_micros: AtomicU64::new(0),
    by_class: [
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
    ],
};

impl Metrics {
    fn observe(&self, seconds: f64, status: StatusCode) {
        match BUCKETS.iter().position(|&b| seconds <= b) {
            Some(i) => &self.buckets[i],
            None => &self.overflow,
        }
        .fetch_add(1, Ordering::Relaxed);
        self.count.fetch_add(1, Ordering::Relaxed);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        self.sum_micros
            .fetch_add((seconds * 1_000_000.0) as u64, Ordering::Relaxed);
        let class = (status.as_u16() / 100).clamp(1, 5) as usize - 1;
        self.by_class[class].fetch_add(1, Ordering::Relaxed);
    }

    /// Prometheus text exposition. Bucket counts become cumulative here.
    fn render(&self) -> String {
        let mut out = String::with_capacity(1024);
        out.push_str("# TYPE fhir_loco_http_requests_total counter\n");
        for (i, c) in self.by_class.iter().enumerate() {
            let v = c.load(Ordering::Relaxed);
            out.push_str(&format!(
                "fhir_loco_http_requests_total{{class=\"{}xx\"}} {v}\n",
                i + 1
            ));
        }
        out.push_str("# TYPE fhir_loco_http_request_duration_seconds histogram\n");
        let mut cumulative = 0u64;
        for (i, bound) in BUCKETS.iter().enumerate() {
            cumulative += self.buckets[i].load(Ordering::Relaxed);
            out.push_str(&format!(
                "fhir_loco_http_request_duration_seconds_bucket{{le=\"{bound}\"}} {cumulative}\n"
            ));
        }
        cumulative += self.overflow.load(Ordering::Relaxed);
        out.push_str(&format!(
            "fhir_loco_http_request_duration_seconds_bucket{{le=\"+Inf\"}} {cumulative}\n"
        ));
        #[allow(clippy::cast_precision_loss)]
        let sum = self.sum_micros.load(Ordering::Relaxed) as f64 / 1_000_000.0;
        out.push_str(&format!(
            "fhir_loco_http_request_duration_seconds_sum {sum}\n"
        ));
        out.push_str(&format!(
            "fhir_loco_http_request_duration_seconds_count {}\n",
            self.count.load(Ordering::Relaxed)
        ));
        out
    }
}

/// Middleware for the FHIR listener: time every request into the histogram.
pub async fn record(request: Request<Body>, next: Next) -> Response {
    let started = Instant::now();
    let response = next.run(request).await;
    METRICS.observe(started.elapsed().as_secs_f64(), response.status());
    response
}

/// The admin router: liveness, readiness, metrics. No FHIR routes, no PHI —
/// nothing here reads a resource, and nothing here may ever start to.
pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/metrics", get(metrics))
}

/// Liveness: the process is up. Says nothing about the store.
async fn health() -> Response {
    (StatusCode::OK, "ok\n").into_response()
}

/// Readiness: the FHIR stores are mounted. A load balancer routes on this;
/// liveness staying green while every request 503s was this crate's original
/// boot bug, which is why the two are separate endpoints.
async fn ready() -> Response {
    match crate::store::versions() {
        Some(v) if !v.mounted().is_empty() => (StatusCode::OK, "ready\n").into_response(),
        _ => (StatusCode::SERVICE_UNAVAILABLE, "no FHIR store mounted\n").into_response(),
    }
}

async fn metrics() -> Response {
    (
        StatusCode::OK,
        [("content-type", "text/plain; version=0.0.4")],
        METRICS.render(),
    )
        .into_response()
}

/// Bind the admin listener if `FHIR_LOCO_ADMIN_BIND` is set.
///
/// Off by default: a second listener nobody asked for is attack surface. When
/// set, the value is a socket address — loopback unless the deployment
/// deliberately chooses otherwise, the same posture as `SV4.4`.
pub fn spawn_if_configured() {
    let Ok(bind) = std::env::var("FHIR_LOCO_ADMIN_BIND") else {
        return;
    };
    let addr: SocketAddr = match bind.parse() {
        Ok(a) => a,
        Err(e) => {
            tracing::error!(%bind, error = %e, "FHIR_LOCO_ADMIN_BIND is not a socket address");
            return;
        }
    };
    tokio::spawn(async move {
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                tracing::info!(%addr, "admin listener up: /health /ready /metrics (SV4.3)");
                if let Err(e) = axum::serve(listener, router()).await {
                    tracing::error!(error = %e, "admin listener failed");
                }
            }
            Err(e) => tracing::error!(%addr, error = %e, "admin listener could not bind"),
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt;

    async fn hit(path: &str) -> (StatusCode, String) {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), 1 << 20)
            .await
            .expect("body");
        (status, String::from_utf8_lossy(&bytes).into_owned())
    }

    #[tokio::test]
    async fn health_is_liveness_only() {
        let (status, body) = hit("/health").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "ok\n");
    }

    #[tokio::test]
    async fn ready_refuses_without_a_mounted_store() {
        // No store is initialised in a unit test, and that is the point:
        // readiness must say so rather than mirror liveness — liveness green
        // over a dead store was this crate's original boot bug.
        let (status, _) = hit("/ready").await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn metrics_speak_prometheus_text() {
        let (status, body) = hit("/metrics").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("# TYPE fhir_loco_http_request_duration_seconds histogram"));
        assert!(body.contains("_bucket{le=\"+Inf\"}"));
    }

    #[test]
    fn buckets_are_cumulative_and_p99_answerable() {
        let m = Metrics::default();
        m.observe(0.003, StatusCode::OK);
        m.observe(0.003, StatusCode::OK);
        m.observe(0.2, StatusCode::OK);
        m.observe(9.0, StatusCode::BAD_GATEWAY);
        let text = m.render();
        // 0.003 lands in le=0.005; the le=0.25 bucket has all three fast ones;
        // +Inf carries the overflow too.
        assert!(text.contains("_bucket{le=\"0.005\"} 2\n"), "{text}");
        assert!(text.contains("_bucket{le=\"0.25\"} 3\n"), "{text}");
        assert!(text.contains("_bucket{le=\"+Inf\"} 4\n"), "{text}");
        assert!(text.contains("_count 4\n"), "{text}");
        assert!(text.contains("requests_total{class=\"2xx\"} 3\n"), "{text}");
        assert!(text.contains("requests_total{class=\"5xx\"} 1\n"), "{text}");
    }
}
