//! (De)serializing [`crossterm`] types
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};

/// A module for use via `#[serde(with)]` for serializing & deserializing
/// [`crossterm::style::ContentStyle`] values as style strings.
///
/// Use it like so:
///
/// ```
/// use serde::{Deserialize, Serialize}
///
/// #[derive(Deserialize, Serialize)]
/// struct MyStruct {
///     #[serde(with = "parse_style::serde::crossterm::content_style")]
///     style: crossterm::style::ContentStyle,
/// }
/// ```
pub mod content_style {
    use super::*;
    use crossterm::style::ContentStyle;

    pub fn serialize<S: Serializer>(
        style: &ContentStyle,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        crate::Style::from(*style).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<ContentStyle, D::Error> {
        crate::Style::deserialize(deserializer).map(ContentStyle::from)
    }
}

/// A module for use via `#[serde(with)]` for serializing & deserializing
/// [`crossterm::style::Color`] values as color words and RGB codes.
///
/// Use it like so:
///
/// ```
/// use serde::{Deserialize, Serialize}
///
/// #[derive(Deserialize, Serialize)]
/// struct MyStruct {
///     #[serde(with = "parse_style::serde::crossterm::color")]
///     color: crossterm::style::Color,
/// }
/// ```
pub mod color {
    use super::*;
    use crossterm::style::Color;

    pub fn serialize<S: Serializer>(color: &Color, serializer: S) -> Result<S::Ok, S::Error> {
        crate::Color::from(*color).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Color, D::Error> {
        crate::Color::deserialize(deserializer).map(Color::from)
    }
}
