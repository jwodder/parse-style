mod ansi_color;
pub use ansi_color::AnsiColor;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("invalid color string: {0:?}")]
pub struct ParseColorError(String);
