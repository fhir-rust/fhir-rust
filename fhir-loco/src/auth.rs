//! Establishing the principal: PASETO v4.public, and nothing else.
//!
//! # Why this exists here and not in a storage crate
//!
//! The storage libraries deliberately do not authenticate — §12 draws that
//! boundary, and every port's README repeats it. But "authentication is
//! elsewhere" must not become "the record of who did what is nowhere"
//! (`PR12.1`–`PR12.4`), and `fhir-loco` is the *elsewhere*: it is the only
//! crate in this repository that terminates a request, so it is the only one
//! that can turn a credential into an [`Audit`].
//!
//! # Why PASETO and not JWT
//!
//! A JWT names its own algorithm in a header field the attacker also controls,
//! which is the root of the `alg: none` and RS256→HS256 confusion families.
//! PASETO fixes the algorithm per version: a `v4.public` token is Ed25519 and
//! cannot claim to be anything else. There is no negotiation to get wrong.
//!
//! # Why v4.public and not v4.local
//!
//! `v4.local` is symmetric: verifying a token and minting one use the same key.
//! Every instance that checks credentials would then be able to issue them, so
//! a single read of the configuration is enough to impersonate any principal.
//! `v4.public` is asymmetric — the issuer signs, this service holds only the
//! public half and **cannot mint a token at all**. For a service handling PHI
//! that difference is the whole point.
//!
//! # Configuration
//!
//! `FHIR_LOCO_PASETO_PUBLIC_KEY` — a 32-byte Ed25519 public key, hex-encoded.
//! It is **required**. Without it the process refuses to boot.
//!
//! Every request MUST carry `Authorization: Bearer v4.public.…`. A missing,
//! malformed, tampered, or expired token is `401`.
//!
//! # There is no header fallback, deliberately
//!
//! An earlier revision accepted a principal from `x-fhir-loco-principal` when
//! no key was configured, on the reasoning that a perimeter proxy sets it. That
//! is removed. Two modes meant the safe one had to be chosen, and the unsafe
//! one was the default — a deployment that forgot the variable ran
//! unauthenticated while looking configured, and the only signal was a log line
//! at boot.
//!
//! A trusted header is also only as trustworthy as the guarantee that nothing
//! else can reach the port. That guarantee lives in network configuration this
//! service cannot see, cannot verify, and cannot fail loudly about. A signature
//! it can check itself is a property of the request rather than of an
//! assumption about the topology.
//!
//! There is likewise no mode that checks a token *if present* and shrugs when
//! absent: that accepts an unauthenticated request whenever the caller omits
//! the header, which is the same as not checking.

use std::sync::OnceLock;

use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response as AxumResponse};
use fhir_sqlite_store::Audit;
use pasetors::claims::ClaimsValidationRules;
use pasetors::keys::AsymmetricPublicKey;
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;
use pasetors::{public, Public};

/// The environment variable holding the hex-encoded Ed25519 public key.
pub const ENV_PUBLIC_KEY: &str = "FHIR_LOCO_PASETO_PUBLIC_KEY";

static VERIFIER: OnceLock<Verifier> = OnceLock::new();

/// How this process establishes a principal: by verifying a PASETO
/// `v4.public` token, and in no other way.
pub struct Verifier(Box<AsymmetricPublicKey<V4>>);

impl Verifier {
    /// Read the configuration once, at boot.
    ///
    /// # Errors
    /// If the key is absent, empty, not hex, or not 32 bytes. All four stop the
    /// process. There is no configuration under which this service starts
    /// without a way to verify who is calling it — an unusable key must not
    /// degrade into trusting the caller.
    pub fn from_env() -> Result<Self, String> {
        let raw = std::env::var(ENV_PUBLIC_KEY).map_err(|_| {
            format!(
                "{ENV_PUBLIC_KEY} is not set. This service authenticates every request \
                 with a PASETO v4.public token and has no unauthenticated mode; set it \
                 to the issuer's 32-byte Ed25519 public key, hex-encoded."
            )
        })?;
        let raw = raw.trim();
        if raw.is_empty() {
            return Err(format!("{ENV_PUBLIC_KEY} is set but empty"));
        }
        let bytes =
            hex::decode(raw).map_err(|e| format!("{ENV_PUBLIC_KEY} is not valid hex: {e}"))?;
        let key = AsymmetricPublicKey::<V4>::from(&bytes)
            .map_err(|e| format!("{ENV_PUBLIC_KEY} is not an Ed25519 public key: {e}"))?;
        Ok(Self(Box::new(key)))
    }
}

/// Install the verifier. Called once from `before_run`.
///
/// # Errors
/// Propagates a malformed key so boot fails loudly.
pub fn init() -> Result<&'static Verifier, String> {
    if let Some(v) = VERIFIER.get() {
        return Ok(v);
    }
    let v = Verifier::from_env()?;
    Ok(VERIFIER.get_or_init(|| v))
}

/// The configured verifier.
///
/// `init` runs in `before_run` and fails the boot if the key is unusable, so
/// reaching a handler without one should be impossible. If it happens anyway,
/// refuse rather than invent a principal.
fn verifier() -> Option<&'static Verifier> {
    VERIFIER.get()
}

/// `401`, with a body that names what was wrong with the *request* and never
/// what is stored — the same rule the store's `Unsupported` error follows.
fn unauthorized(detail: &str) -> AxumResponse {
    (
        StatusCode::UNAUTHORIZED,
        [("www-authenticate", "Bearer")],
        detail.to_string(),
    )
        .into_response()
}

/// The bearer token, if the header is well-formed.
fn bearer(headers: &HeaderMap) -> Option<&str> {
    let v = headers
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?;
    let (scheme, token) = v.split_once(' ')?;
    scheme.eq_ignore_ascii_case("bearer").then(|| token.trim())
}

/// Who is responsible for this request.
///
/// # Errors
/// `401` when PASETO is enforced and the token is absent, malformed, tampered
/// with, or expired.
// Same convention as `version_of`, `if_match` and `parse_body` in the fhir
// controller: the error *is* the response, which is the point — boxing it would
// buy a few bytes on a path that is about to write an HTTP response anyway.
#[allow(clippy::result_large_err)]
pub fn audit_from(headers: &HeaderMap) -> Result<Audit, AxumResponse> {
    let Some(v) = verifier() else {
        return Err(unauthorized(
            "authentication is not configured on this server",
        ));
    };
    let Some(token) = bearer(headers) else {
        return Err(unauthorized("a bearer token is required"));
    };
    let actor = verify_v4_public(&v.0, token)
        .map_err(|why| unauthorized(&format!("token rejected: {why}")))?;

    Ok(Audit {
        actor,
        actor_source: Some("paseto:v4.public".to_string()),
        client: headers
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .map(str::to_string),
        request_id: headers
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .map(str::to_string),
        reason: headers
            .get("x-fhir-loco-reason")
            .and_then(|v| v.to_str().ok())
            .map(str::to_string),
    })
}

/// Verify a `v4.public` token and return its `sub` claim.
///
/// The default [`ClaimsValidationRules`] enforce `exp` and `nbf` when present.
/// The rejection reason is returned for the log and for the response body: it
/// describes the *token the caller sent*, so it discloses nothing about stored
/// data.
fn verify_v4_public(key: &AsymmetricPublicKey<V4>, token: &str) -> Result<String, String> {
    let untrusted =
        UntrustedToken::<Public, V4>::try_from(token).map_err(|e| format!("malformed: {e}"))?;
    let rules = ClaimsValidationRules::new();
    let trusted = public::verify(key, &untrusted, &rules, None, None)
        .map_err(|e| format!("signature or claims invalid: {e}"))?;
    let claims = trusted
        .payload_claims()
        .ok_or_else(|| "no claims".to_string())?;
    let sub = claims
        .get_claim("sub")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "no `sub` claim to attribute the change to".to_string())?;
    if sub.trim().is_empty() {
        return Err("`sub` claim is empty".to_string());
    }
    Ok(sub.to_string())
}

/// The listener's transport posture (`SV3.11`).
///
/// This service speaks plain HTTP. A loopback bind keeps that inside the
/// host; a non-loopback bind puts PHI on a network in the clear unless a
/// TLS-terminating proxy sits in front — and only the deployment knows
/// whether one does. So a non-loopback bind MUST be accompanied by the
/// explicit acknowledgement `FHIR_LOCO_TLS_TERMINATED_UPSTREAM=true`, or the
/// boot refuses: PHI in the clear must be a choice someone made, not a
/// default nobody noticed. The same shape as `O10.7`'s database-side rule.
pub fn listener_posture(binding: &str, tls_terminated_upstream: bool) -> Result<(), String> {
    let loopback = binding == "localhost"
        || binding
            .parse::<std::net::IpAddr>()
            .is_ok_and(|ip| ip.is_loopback());
    if loopback || tls_terminated_upstream {
        return Ok(());
    }
    Err(format!(
        "refusing to bind {binding}: this listener speaks plain HTTP and the \
         bind is not loopback. Either terminate TLS in front and set \
         FHIR_LOCO_TLS_TERMINATED_UPSTREAM=true to say so, or bind loopback \
         (SV3.11)."
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pasetors::claims::Claims;
    use pasetors::keys::{AsymmetricKeyPair, Generate};

    fn signed(sub: &str) -> (AsymmetricKeyPair<V4>, String) {
        let kp = AsymmetricKeyPair::<V4>::generate().expect("keypair");
        let mut claims = Claims::new().expect("claims");
        claims.subject(sub).expect("sub");
        let token = public::sign(&kp.secret, &claims, None, None).expect("sign");
        (kp, token)
    }

    #[test]
    fn a_valid_token_yields_its_subject() {
        let (kp, token) = signed("practitioner/42");
        assert_eq!(
            verify_v4_public(&kp.public, &token).expect("verifies"),
            "practitioner/42"
        );
    }

    /// The property that matters. Mutation-verified (`T11.10`): removing the
    /// signature check in `verify_v4_public` makes this pass, which is how a
    /// forged attribution would reach the audit chain looking authentic.
    #[test]
    fn a_token_signed_by_another_key_is_rejected() {
        let (_issuer, token) = signed("practitioner/42");
        let attacker = AsymmetricKeyPair::<V4>::generate().expect("keypair");
        assert!(verify_v4_public(&attacker.public, &token).is_err());
    }

    #[test]
    fn a_tampered_payload_is_rejected() {
        let (kp, token) = signed("practitioner/42");
        // Flip one character of the payload segment.
        let mut parts: Vec<&str> = token.split('.').collect();
        let payload = parts[2].to_string();
        let flipped: String = payload
            .chars()
            .enumerate()
            .map(|(i, c)| if i == 0 && c != 'A' { 'A' } else { c })
            .collect();
        parts[2] = &flipped;
        assert!(verify_v4_public(&kp.public, &parts.join(".")).is_err());
    }

    #[test]
    fn a_token_without_a_subject_is_rejected() {
        let kp = AsymmetricKeyPair::<V4>::generate().expect("keypair");
        let claims = Claims::new().expect("claims"); // no subject
        let token = public::sign(&kp.secret, &claims, None, None).expect("sign");
        let e = verify_v4_public(&kp.public, &token).expect_err("must reject");
        assert!(
            e.contains("sub"),
            "reason should name the missing claim: {e}"
        );
    }

    #[test]
    fn a_v4_local_token_is_not_accepted_as_public() {
        let kp = AsymmetricKeyPair::<V4>::generate().expect("keypair");
        // A `v4.local.…` shape must not verify as public: the version and
        // purpose are part of the token, which is the property JWT lacks.
        assert!(verify_v4_public(&kp.public, "v4.local.aaaaaaaaaaaaaaaa").is_err());
    }

    #[test]
    fn bearer_parsing_is_scheme_insensitive_and_ignores_others() {
        let mut h = HeaderMap::new();
        h.insert("authorization", "BeArEr abc".parse().expect("hv"));
        assert_eq!(bearer(&h), Some("abc"));
        h.insert("authorization", "Basic abc".parse().expect("hv"));
        assert_eq!(bearer(&h), None);
    }

    #[test]
    fn a_malformed_key_is_a_boot_failure_not_a_silent_downgrade() {
        // The dangerous outcome would be falling back to header mode.
        let bad = hex::decode("zz");
        assert!(bad.is_err());
        let short = AsymmetricPublicKey::<V4>::from(&[0u8; 8]);
        assert!(short.is_err(), "an 8-byte key must not be accepted");
    }

    #[test]
    fn loopback_binds_need_no_acknowledgement() {
        assert!(listener_posture("127.0.0.1", false).is_ok());
        assert!(listener_posture("::1", false).is_ok());
        assert!(listener_posture("localhost", false).is_ok());
    }

    #[test]
    fn a_non_loopback_plaintext_bind_refuses_without_the_acknowledgement() {
        let e = listener_posture("0.0.0.0", false).expect_err("must refuse");
        assert!(
            e.contains("SV3.11"),
            "the refusal names its requirement: {e}"
        );
        assert!(
            e.contains("FHIR_LOCO_TLS_TERMINATED_UPSTREAM"),
            "the refusal says how to fix it: {e}"
        );
        // With the acknowledgement, the deployment has made the choice.
        assert!(listener_posture("0.0.0.0", true).is_ok());
        // A hostname is not provably loopback; it needs the acknowledgement too.
        assert!(listener_posture("fhir.internal", false).is_err());
    }
}
