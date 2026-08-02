//! `fhir-store` — the engine-agnostic half of FHIR persistence.
//!
//! Everything about storing FHIR that is **not** specific to one SQL engine:
//! the tamper-evident audit chain, the attribution and disclosure records it
//! commits to, and the value types every port's operations return.
//!
//! # Why this crate exists
//!
//! Six ports — `fhir-postgresql`, `fhir-sqlite`, `fhir-mysql`, `fhir-mariadb`,
//! `fhir-mssql`, `fhir-oracle` — each carried their own copy of this code.
//! `chain.rs` alone was **618 lines, byte-identical in all six**, and unlike
//! the map and generator it was not covered by
//! `scripts/check-shared-core.sh`: the gate watched `map/` and `gen/` and left
//! the store's engine-agnostic half unwatched. That is why closing **F-07**
//! (the hash-chain pre-image was derived from `jsonb`, so chains were not
//! portable) had to be done six times by hand.
//!
//! What stays in a port is what genuinely differs: the driver, transactions,
//! placeholder syntax, the search-SQL builder, and `StoreError` — which wraps
//! that port's own `ShredError` and so cannot be lifted here.
//!
//! # What this crate does not do
//!
//! It does not authenticate. That is the perimeter's job, and this crate holds
//! the boundary rather than crossing it (§12, `PR12.1`–`PR12.4`): "authentication
//! is elsewhere" must not become "the record of who did what is nowhere", so
//! the types below carry attribution and the chain makes it tamper-evident.
//!
//! It also opens no sockets and speaks no HTTP. The RESTful surface is
//! `fhir-loco`.

pub mod chain;

use serde_json::Value;

#[derive(Debug)]
pub struct PutOutcome {
    pub id: String,
    pub version_id: i64,
    pub created: bool,
}

/// Who is responsible for a change, and how we know (spec M3.15, PR12.1–4).
///
/// fhir does not authenticate — that is the perimeter's job (plan D13) —
/// but "authentication is elsewhere" cannot mean "the record of who did what
/// is nowhere" (D15). The perimeter knows the identity; only the store knows
/// which rows were touched, so only the store can join the two.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Audit {
    /// The authenticated principal, or `unauthenticated`.
    pub actor: String,
    /// How the principal was established, e.g. `header:X-Fhir-Principal`
    /// or `cli`. Recorded so a reader can weigh how much the actor is worth.
    pub actor_source: Option<String>,
    /// Source address as the server observed it.
    pub client: Option<String>,
    /// The value echoed in `X-Request-Id`, tying this row to the logs.
    pub request_id: Option<String>,
    /// Caller-supplied purpose of use.
    pub reason: Option<String>,
}

impl Default for Audit {
    fn default() -> Self {
        Self::unattributed()
    }
}

impl Audit {
    /// A write with no identity behind it. Recorded as such rather than left
    /// blank: "we do not know who did this" is itself an audit finding.
    #[must_use]
    pub fn unattributed() -> Self {
        Audit {
            actor: "unauthenticated".to_string(),
            actor_source: None,
            client: None,
            request_id: None,
            reason: None,
        }
    }

    /// A write by the operator running the CLI on this host.
    #[must_use]
    pub fn cli() -> Self {
        let who = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string());
        Audit {
            actor: format!("cli:{who}"),
            actor_source: Some("cli".to_string()),
            client: None,
            request_id: None,
            reason: None,
        }
    }

    /// A write attributed to a principal the perimeter vouched for.
    #[must_use]
    pub fn principal(actor: impl Into<String>, source: impl Into<String>) -> Self {
        Audit {
            actor: actor.into(),
            actor_source: Some(source.into()),
            client: None,
            request_id: None,
            reason: None,
        }
    }

    #[must_use]
    pub fn with_client(mut self, client: Option<String>) -> Self {
        self.client = client;
        self
    }

    #[must_use]
    pub fn with_request_id(mut self, request_id: Option<String>) -> Self {
        self.request_id = request_id;
        self
    }

    #[must_use]
    pub fn with_reason(mut self, reason: Option<String>) -> Self {
        self.reason = reason;
        self
    }
}

/// One disclosure record (spec PR12.5).
#[derive(Debug, Clone)]
pub struct AccessRecord {
    pub audit: Audit,
    /// `read`, `vread`, `history`, `search`, `export`.
    pub interaction: String,
    pub rtype: Option<String>,
    pub id: Option<String>,
    pub version_id: Option<i64>,
    /// `ok`, `not-found`, `denied`, `error`.
    pub outcome: String,
    pub result_count: Option<i64>,
}

/// What an erasure removed (spec M3.18).
#[derive(Debug, PartialEq, Eq)]
pub struct PurgeReport {
    /// History rows removed, not counting the tombstone left behind.
    pub versions_erased: u64,
    /// Whether the resource was known at all.
    pub existed: bool,
}

/// A break in one resource's hash chain (spec M3.16, M3.16a).
///
/// One break per algorithm, never merged: a regime that recognises only
/// SHA-3 must be able to see the SHA-3 verdict on its own, and a break that
/// appears under one algorithm but not the other is itself worth seeing —
/// it means the stored digests disagree about the same bytes.
/// `#[non_exhaustive]` so that a future algorithm or field is a patch, not a
/// break: adding a public field to a struct callers can build or destructure
/// exhaustively is a breaking change, and this type gained `algorithm`
/// exactly that way.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct ChainBreak {
    pub rtype: String,
    pub id: String,
    pub version_id: i64,
    /// `"sha256"` or `"sha3-256"`.
    pub algorithm: &'static str,
    pub detail: String,
}

impl ChainBreak {
    /// Record a break.
    ///
    /// A constructor rather than a struct literal because this type is
    /// `#[non_exhaustive]`: it gained `algorithm` after release, and keeping
    /// that freedom is the point. A literal can only be written inside this
    /// crate, and the ports that detect breaks are outside it, so without this
    /// they would have to choose between constructing the type and keeping the
    /// guarantee.
    #[must_use]
    pub fn new(
        rtype: impl Into<String>,
        id: impl Into<String>,
        version_id: i64,
        algorithm: &'static str,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            rtype: rtype.into(),
            id: id.into(),
            version_id,
            algorithm,
            detail: detail.into(),
        }
    }
}

#[derive(Debug)]
pub struct UpgradeReport {
    pub additive: usize,
    pub destructive: usize,
    /// Distinct string values folded into new search columns (P6.6).
    pub folded: usize,
}

#[derive(Debug)]
pub struct SearchOutcome {
    pub ids: Vec<String>,
    pub total: Option<i64>,
}

/// The result of a conditional create (`If-None-Exist`).
#[derive(Debug)]
pub enum CondCreate {
    /// No match: the resource was created.
    Created(PutOutcome),
    /// Exactly one match: it is returned unchanged, per FHIR.
    Existing(String),
    /// More than one match: the criteria are not selective enough (412).
    Multiple,
}

/// The result of a conditional delete.
#[derive(Debug, PartialEq, Eq)]
pub enum CondDelete {
    Deleted,
    NoMatch,
    Multiple,
}

#[derive(Debug)]
pub struct Got {
    pub resource: Value,
    pub version_id: i64,
}

/// One write inside a FHIR transaction Bundle.
#[derive(Debug)]
pub enum TxOp {
    Put {
        resource: Value,
        expected: Option<i64>,
    },
    Delete {
        rtype: String,
        id: String,
    },
}

#[derive(Debug)]
pub enum TxOutcome {
    Put(PutOutcome),
    Delete(bool),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResourceStatus {
    Active(i64),
    /// Deleted; carries the delete marker's version.
    Deleted(i64),
    Unknown,
}

#[derive(Debug)]
pub struct HistEntry {
    pub version_id: i64,
    pub last_updated: String,
    /// 'C' create, 'U' update, 'D' delete.
    pub op: char,
    pub resource: Option<Value>,
}
