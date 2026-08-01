use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShredError {
    #[error("{path}: {msg}")]
    At { path: String, msg: String },
    /// Stored rows are inconsistent with the map — indicates corruption or a
    /// map/schema mismatch, never a caller mistake.
    #[error("integrity: {0}")]
    Integrity(String),
}

impl ShredError {
    pub fn at(path: impl Into<String>, msg: impl Into<String>) -> Self {
        ShredError::At {
            path: path.into(),
            msg: msg.into(),
        }
    }

    pub fn integrity(msg: impl Into<String>) -> Self {
        ShredError::Integrity(msg.into())
    }
}
