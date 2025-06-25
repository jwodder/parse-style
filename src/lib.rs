mod color256;
pub use color256::Color256;
use thiserror::Error;

/// Error returned when parsing a color string fails
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("invalid color string: {0:?}")]
pub struct ParseColorError(
    /// The invalid color string
    pub String,
);
