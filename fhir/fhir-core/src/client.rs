//! An async FHIR REST client (feature `client`).
//!
//! [`Client`] wraps a [`reqwest`] client and speaks the FHIR RESTful API:
//! `read`, `vread`, `create`, `update`, `delete`, `search`, and `capabilities`.
//! Non-success responses are surfaced as [`ClientError`], parsing the server's
//! `OperationOutcome` when present.
//!
//! ```no_run
//! # async fn demo() -> Result<(), fhir::client::ClientError> {
//! use fhir::client::Client;
//!
//! let client = Client::new("https://hapi.fhir.org/baseR5");
//! let patient = client.read("Patient", "example").await?;
//! println!("{patient:?}");
//! # Ok(()) }
//! ```
//!
//! # Choosing a release
//!
//! The wire protocol is the same for every FHIR release; only the resource
//! types differ. [`ReleaseClient<R>`] is therefore generic over a
//! [`Release`](crate::release::Release), and each release module exposes an
//! alias for it: [`Client`] here (and [`r5::client::Client`](crate::r5::client))
//! for R5, [`r4::client::Client`](crate::r4::client) for R4.
//!
//! ```no_run
//! # // Gated: this module compiles whenever `client` is on, but the example
//! # // names an R4 type. Without the cfg the doctest fails to compile under
//! # // `--features client` alone, which is a real feature combination.
//! # #[cfg(feature = "r4")]
//! # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
//! let r4 = fhir::r4::client::Client::new("https://hapi.fhir.org/baseR4");
//! let bundle = r4.search("Patient", &[("name", "chalmers")]).await?;
//! # Ok(()) }
//! ```

use ::serde::Serialize;

use crate::release::Release;

/// FHIR JSON media type.
const FHIR_JSON: &str = "application/fhir+json";

/// Whether a failure is worth retrying: transport trouble, or a server that
/// said it could not answer *this time*. A 4xx is the client's fault and will
/// fail identically on a second attempt.
fn is_retryable<R: Release>(e: &ReleaseClientError<R>) -> bool {
    match e {
        ReleaseClientError::Http(e) => e.is_timeout() || e.is_connect() || e.is_request(),
        ReleaseClientError::Outcome { status, .. } | ReleaseClientError::Status { status, .. } => {
            *status >= 500 || *status == 429
        }
        ReleaseClientError::Url(_) | ReleaseClientError::BodyTooLarge { .. } => false,
    }
}

/// An error from a FHIR REST interaction with release `R`.
///
/// Most code wants the release-specific alias — [`ClientError`] for R5, or
/// [`r4::client::ClientError`](crate::r4::client) for R4.
pub enum ReleaseClientError<R: Release> {
    /// A transport or (de)serialization error from `reqwest`.
    Http(reqwest::Error),
    /// The server returned an error status with an `OperationOutcome` body.
    Outcome {
        /// HTTP status code.
        status: u16,
        /// The parsed outcome.
        outcome: Box<R::OperationOutcome>,
    },
    /// The server returned an error status without a parseable `OperationOutcome`.
    Status {
        /// HTTP status code.
        status: u16,
        /// The response body, truncated. **May contain PHI** — a server that
        /// failed to produce an `OperationOutcome` may have echoed the
        /// resource instead, so do not log this at large (spec R13.10).
        body: String,
    },
    /// The resource type or id could not be made into a URL.
    Url(String),
    /// The response exceeded the configured body cap (spec R13.5).
    BodyTooLarge {
        /// The cap that was exceeded, in bytes.
        limit: usize,
    },
}

// Written by hand rather than derived: `#[derive(Debug)]` would demand
// `R: Debug` of the release marker, which says nothing about whether the error
// can be printed.
// `body` is omitted from `Debug` deliberately: it can hold a resource, and
// `Debug` output reaches logs and panic messages (spec R13.10).
#[allow(clippy::missing_fields_in_debug)]
impl<R: Release> std::fmt::Debug for ReleaseClientError<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReleaseClientError::Http(e) => f.debug_tuple("Http").field(e).finish(),
            ReleaseClientError::Outcome { status, outcome } => f
                .debug_struct("Outcome")
                .field("status", status)
                .field("outcome", outcome)
                .finish(),
            // The body is deliberately not printed: `Debug` output ends up in
            // logs and panic messages, and this field may hold a resource
            // (spec R13.10).
            ReleaseClientError::Status { status, body } => f
                .debug_struct("Status")
                .field("status", status)
                .field("body_len", &body.len())
                .finish_non_exhaustive(),
            ReleaseClientError::Url(msg) => f.debug_tuple("Url").field(msg).finish(),
            ReleaseClientError::BodyTooLarge { limit } => f
                .debug_struct("BodyTooLarge")
                .field("limit", limit)
                .finish(),
        }
    }
}

impl<R: Release> std::fmt::Display for ReleaseClientError<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReleaseClientError::Http(e) => write!(f, "HTTP error: {e}"),
            ReleaseClientError::Outcome { status, .. } => {
                write!(f, "FHIR error status {status} (OperationOutcome)")
            }
            ReleaseClientError::Status { status, .. } => write!(f, "error status {status}"),
            ReleaseClientError::Url(msg) => write!(f, "cannot build request URL: {msg}"),
            ReleaseClientError::BodyTooLarge { limit } => {
                write!(f, "response body exceeds {limit} bytes")
            }
        }
    }
}

impl<R: Release> std::error::Error for ReleaseClientError<R> {}

impl<R: Release> From<reqwest::Error> for ReleaseClientError<R> {
    fn from(e: reqwest::Error) -> Self {
        ReleaseClientError::Http(e)
    }
}

/// An async FHIR REST client for a single service base URL, speaking release `R`.
///
/// Most code wants the release-specific alias — [`Client`] for R5, or
/// [`r4::client::Client`](crate::r4::client) for R4.
/// Defaults chosen so that a client built with [`ReleaseClient::new`] is safe
/// to point at a network you do not control (spec R13.5).
const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
const DEFAULT_CONNECT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);
/// 64 MiB: larger than any single resource has business being, smaller than
/// an amount of memory a hostile or broken peer should be able to make us
/// allocate.
const DEFAULT_MAX_BODY: usize = 64 * 1024 * 1024;

/// How many times an idempotent request is retried, and how long it waits.
///
/// Only `GET`, `PUT`, and `DELETE` are retried: FHIR `POST` creates a new
/// resource each time, so retrying one after a timeout is how a patient ends
/// up in the chart twice (spec R13.8).
#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
    /// Additional attempts after the first. Zero disables retrying.
    pub attempts: u32,
    /// Delay before the first retry; doubled each time.
    pub backoff: std::time::Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            attempts: 0,
            backoff: std::time::Duration::from_millis(200),
        }
    }
}

/// Supplies a bearer token per request, so credentials are not baked into a
/// hand-built `reqwest::Client` and can be refreshed (spec R13.9).
type TokenSource = std::sync::Arc<dyn Fn() -> Option<String> + Send + Sync>;

#[derive(Clone)]
pub struct ReleaseClient<R: Release> {
    base_url: String,
    http: reqwest::Client,
    auth: Option<TokenSource>,
    retry: RetryPolicy,
    max_body: usize,
    release: std::marker::PhantomData<R>,
}

// As with the error type, deriving would impose a needless `R: Debug`.
impl<R: Release> std::fmt::Debug for ReleaseClient<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReleaseClient")
            .field("base_url", &self.base_url)
            .field("http", &self.http)
            .field("release", &R::LABEL)
            // Whether a token supplier is configured, never what it returns:
            // this is a debug dump, and a bearer token is a credential.
            .field("auth", &self.auth.as_ref().map(|_| "<token supplier>"))
            .field("retry", &self.retry)
            .field("max_body", &self.max_body)
            .finish()
    }
}

impl<R: Release> ReleaseClient<R> {
    /// A client for the given service base URL (e.g. `https://.../baseR5`).
    /// A client for the given service base URL (e.g. `https://.../baseR5`),
    /// with request and connect timeouts already set.
    ///
    /// `reqwest::Client::new()` has *no* timeout, so a client built on it
    /// waits forever on a server that accepts the connection and then stops
    /// talking — which is what a stalled FHIR server looks like from the
    /// outside (spec R13.5).
    #[must_use]
    pub fn new(base_url: impl Into<String>) -> Self {
        let http = reqwest::Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
            .build()
            .unwrap_or_default();
        Self::with_http(base_url, http)
    }

    /// A client using a caller-provided `reqwest::Client` (for custom TLS,
    /// timeouts, proxies, …).
    ///
    /// The caller owns the timeout policy here: nothing is added to a client
    /// you built yourself.
    #[must_use]
    pub fn with_http(base_url: impl Into<String>, http: reqwest::Client) -> Self {
        let base_url = base_url.into().trim_end_matches('/').to_string();
        Self {
            base_url,
            http,
            auth: None,
            retry: RetryPolicy::default(),
            max_body: DEFAULT_MAX_BODY,
            release: std::marker::PhantomData,
        }
    }

    /// Attach a bearer-token supplier, called once per request so a token can
    /// be refreshed without rebuilding the client.
    #[must_use]
    pub fn with_bearer_token<F>(mut self, source: F) -> Self
    where
        F: Fn() -> Option<String> + Send + Sync + 'static,
    {
        self.auth = Some(std::sync::Arc::new(source));
        self
    }

    /// Retry idempotent requests (`GET`, `PUT`, `DELETE`) on transport
    /// failure or 5xx, with exponential backoff.
    #[must_use]
    pub fn with_retry(mut self, retry: RetryPolicy) -> Self {
        self.retry = retry;
        self
    }

    /// Cap the response body this client will buffer.
    #[must_use]
    pub fn with_max_body(mut self, bytes: usize) -> Self {
        self.max_body = bytes;
        self
    }

    /// Build a request URL, percent-encoding each path segment.
    ///
    /// Interpolating an id straight into a URL lets `../Patient/other` — or
    /// anything containing `?` or `#` — address a different interaction than
    /// the caller asked for (spec R13.6).
    fn url(&self, segments: &[&str]) -> Result<reqwest::Url, ReleaseClientError<R>> {
        let mut url = reqwest::Url::parse(&self.base_url)
            .map_err(|e| ReleaseClientError::Url(format!("{}: {e}", self.base_url)))?;
        {
            let mut path = url.path_segments_mut().map_err(|()| {
                ReleaseClientError::Url("base URL cannot have path segments".into())
            })?;
            for s in segments {
                path.push(s);
            }
        }
        Ok(url)
    }

    /// Send a request: attach auth and `Accept`, retry when the policy allows
    /// it, and turn a non-success status into a [`ReleaseClientError`]
    /// (parsing an `OperationOutcome` from the body when possible).
    async fn send(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, ReleaseClientError<R>> {
        let idempotent = req
            .try_clone()
            .and_then(|r| r.build().ok())
            .is_some_and(|r| {
                matches!(
                    *r.method(),
                    reqwest::Method::GET | reqwest::Method::PUT | reqwest::Method::DELETE
                )
            });
        let mut delay = self.retry.backoff;
        let tries = if idempotent { self.retry.attempts } else { 0 };
        for attempt in 0..=tries {
            let Some(this) = req.try_clone() else {
                // A streaming body cannot be replayed; send once.
                return self.send_once(req).await;
            };
            match self.send_once(this).await {
                Ok(resp) => return Ok(resp),
                Err(e) if attempt < tries && is_retryable(&e) => {
                    tokio::time::sleep(delay).await;
                    delay *= 2;
                }
                Err(e) => return Err(e),
            }
        }
        self.send_once(req).await
    }

    async fn send_once(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, ReleaseClientError<R>> {
        let mut req = req.header(reqwest::header::ACCEPT, FHIR_JSON);
        if let Some(source) = &self.auth
            && let Some(token) = source()
        {
            req = req.bearer_auth(token);
        }
        let resp = req.send().await?;
        if resp.status().is_success() {
            return Ok(resp);
        }
        let status = resp.status().as_u16();
        let body = self.body_capped(resp).await?;
        match ::serde_json::from_str::<R::OperationOutcome>(&body) {
            Ok(outcome) => Err(ReleaseClientError::Outcome {
                status,
                outcome: Box::new(outcome),
            }),
            Err(_) => Err(ReleaseClientError::Status {
                status,
                // Truncated: an error body from a non-conformant server may
                // be the resource itself (spec R13.10).
                body: body.chars().take(2048).collect(),
            }),
        }
    }

    /// Buffer a response body, refusing to grow past the configured cap.
    ///
    /// `resp.text()` would happily allocate whatever a peer sends.
    async fn body_capped(
        &self,
        mut resp: reqwest::Response,
    ) -> Result<String, ReleaseClientError<R>> {
        if resp
            .content_length()
            .is_some_and(|n| n > self.max_body as u64)
        {
            return Err(ReleaseClientError::BodyTooLarge {
                limit: self.max_body,
            });
        }
        let mut buf: Vec<u8> = Vec::new();
        while let Some(chunk) = resp.chunk().await? {
            if buf.len() + chunk.len() > self.max_body {
                return Err(ReleaseClientError::BodyTooLarge {
                    limit: self.max_body,
                });
            }
            buf.extend_from_slice(&chunk);
        }
        String::from_utf8(buf).map_err(|e| ReleaseClientError::Url(e.to_string()))
    }

    /// Deserialize a success response, honoring the body cap.
    async fn json<T: ::serde::de::DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T, ReleaseClientError<R>> {
        let body = self.body_capped(resp).await?;
        ::serde_json::from_str(&body).map_err(|e| ReleaseClientError::Status {
            status: 200,
            body: format!("malformed FHIR JSON: {e}"),
        })
    }

    /// `GET [base]/[type]/[id]` — read the current version of a resource.
    pub async fn read(
        &self,
        resource_type: &str,
        id: &str,
    ) -> Result<R::Resource, ReleaseClientError<R>> {
        Ok(self.read_with_etag(resource_type, id).await?.0)
    }

    /// `read`, also returning the response `ETag`.
    ///
    /// The ETag is what a later `update_if_match` needs, so a read-then-write
    /// cycle can be safe against a concurrent writer (spec R13.7). Without
    /// it, every update is last-write-wins.
    pub async fn read_with_etag(
        &self,
        resource_type: &str,
        id: &str,
    ) -> Result<(R::Resource, Option<String>), ReleaseClientError<R>> {
        let url = self.url(&[resource_type, id])?;
        let resp = self.send(self.http.get(url)).await?;
        let etag = resp
            .headers()
            .get(reqwest::header::ETAG)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string);
        Ok((self.json(resp).await?, etag))
    }

    /// `GET [base]/[type]/[id]/_history/[vid]` — read a specific version.
    pub async fn vread(
        &self,
        resource_type: &str,
        id: &str,
        version_id: &str,
    ) -> Result<R::Resource, ReleaseClientError<R>> {
        let url = self.url(&[resource_type, id, "_history", version_id])?;
        let resp = self.send(self.http.get(url)).await?;
        self.json(resp).await
    }

    /// `POST [base]/[type]` — create a resource; returns the server's copy.
    pub async fn create<T: Serialize>(
        &self,
        resource_type: &str,
        resource: &T,
    ) -> Result<R::Resource, ReleaseClientError<R>> {
        let url = self.url(&[resource_type])?;
        let resp = self.send(self.http.post(url).json(resource)).await?;
        self.json(resp).await
    }

    /// `POST [base]/[type]` with `If-None-Exist` — create only if the search
    /// criteria match nothing.
    ///
    /// The server answers 201 with the new resource, or 200 with the existing
    /// one. This is how a client avoids creating a duplicate patient when a
    /// previous attempt's response was lost (spec R13.7).
    pub async fn create_conditional<T: Serialize>(
        &self,
        resource_type: &str,
        resource: &T,
        if_none_exist: &str,
    ) -> Result<R::Resource, ReleaseClientError<R>> {
        let url = self.url(&[resource_type])?;
        let resp = self
            .send(
                self.http
                    .post(url)
                    .header("If-None-Exist", if_none_exist)
                    .json(resource),
            )
            .await?;
        self.json(resp).await
    }

    /// `PUT [base]/[type]/[id]` — update (or create) a resource at a known id.
    pub async fn update<T: Serialize>(
        &self,
        resource_type: &str,
        id: &str,
        resource: &T,
    ) -> Result<R::Resource, ReleaseClientError<R>> {
        self.put(resource_type, id, resource, None).await
    }

    /// `update`, but only if the server's current version still matches
    /// `etag` — otherwise the server answers 412 and nothing is overwritten.
    pub async fn update_if_match<T: Serialize>(
        &self,
        resource_type: &str,
        id: &str,
        resource: &T,
        etag: &str,
    ) -> Result<R::Resource, ReleaseClientError<R>> {
        self.put(resource_type, id, resource, Some(etag)).await
    }

    async fn put<T: Serialize>(
        &self,
        resource_type: &str,
        id: &str,
        resource: &T,
        etag: Option<&str>,
    ) -> Result<R::Resource, ReleaseClientError<R>> {
        let url = self.url(&[resource_type, id])?;
        let mut req = self.http.put(url).json(resource);
        if let Some(etag) = etag {
            req = req.header(reqwest::header::IF_MATCH, etag);
        }
        let resp = self.send(req).await?;
        self.json(resp).await
    }

    /// `DELETE [base]/[type]/[id]`.
    pub async fn delete(&self, resource_type: &str, id: &str) -> Result<(), ReleaseClientError<R>> {
        let url = self.url(&[resource_type, id])?;
        self.send(self.http.delete(url)).await?;
        Ok(())
    }

    /// `delete`, but only if the server's current version still matches.
    pub async fn delete_if_match(
        &self,
        resource_type: &str,
        id: &str,
        etag: &str,
    ) -> Result<(), ReleaseClientError<R>> {
        let url = self.url(&[resource_type, id])?;
        self.send(
            self.http
                .delete(url)
                .header(reqwest::header::IF_MATCH, etag),
        )
        .await?;
        Ok(())
    }

    /// `GET [base]/[type]?[params]` — search, returning the first page.
    pub async fn search(
        &self,
        resource_type: &str,
        params: &[(&str, &str)],
    ) -> Result<R::Bundle, ReleaseClientError<R>> {
        let url = self.url(&[resource_type])?;
        let resp = self.send(self.http.get(url).query(params)).await?;
        self.json(resp).await
    }

    /// Follow a searchset's `next` link, if it has one.
    ///
    /// Returns `None` at the last page, so a caller can loop without parsing
    /// links itself.
    pub async fn next_page(
        &self,
        bundle: &R::Bundle,
    ) -> Result<Option<R::Bundle>, ReleaseClientError<R>> {
        let Some(next) = R::next_link(bundle) else {
            return Ok(None);
        };
        // The link is a server-supplied absolute URL; it is used as given.
        let resp = self.send(self.http.get(next)).await?;
        Ok(Some(self.json(resp).await?))
    }

    /// Search and follow `next` links, collecting up to `max_pages` bundles.
    ///
    /// Bounded on purpose: a server whose paging never terminates should cost
    /// a caller a known number of requests, not an unbounded loop.
    pub async fn search_all(
        &self,
        resource_type: &str,
        params: &[(&str, &str)],
        max_pages: usize,
    ) -> Result<Vec<R::Bundle>, ReleaseClientError<R>> {
        let mut pages = Vec::new();
        let mut current = self.search(resource_type, params).await?;
        loop {
            let next = self.next_page(&current).await?;
            pages.push(current);
            match next {
                Some(b) if pages.len() < max_pages => current = b,
                _ => return Ok(pages),
            }
        }
    }

    /// `GET [base]/metadata` — the server's `CapabilityStatement`.
    pub async fn capabilities(&self) -> Result<R::CapabilityStatement, ReleaseClientError<R>> {
        let url = self.url(&["metadata"])?;
        let resp = self.send(self.http.get(url)).await?;
        self.json(resp).await
    }
}
