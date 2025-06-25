mod color256;
pub use color256::Color256;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("invalid color string: {0:?}")]
pub struct ParseColorError(String);
