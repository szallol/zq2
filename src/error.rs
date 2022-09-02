use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZqError {
    #[error("Failed to import source: {0}")]
    FailedSource(String),
    #[error("Storrage error")]
    StorrageError(#[from] rusqlite::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
