mod attributes;
mod color;
mod color256;
mod rgbcolor;
mod style;
pub use crate::attributes::{Attribute, AttributeSet, ParseAttributeError};
pub use crate::color::Color;
pub use crate::color256::Color256;
pub use crate::rgbcolor::RgbColor;
pub use crate::style::Style;
use thiserror::Error;

/// Error returned when parsing a color string fails
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("invalid color string: {0:?}")]
pub struct ParseColorError(
    /// The invalid color string
    pub String,
);
