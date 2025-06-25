//! (De)serializing [`ratatui`] types
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};

/// A module for use via `#[serde(with)]` for serializing & deserializing
/// [`ratatui::style::Style`] values as style strings.
///
/// Use it like so:
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Serialize)]
/// struct MyStruct {
///     #[serde(with = "parse_style::serde::ratatui::style")]
///     style: ratatui::style::Style,
/// }
/// ```
pub mod style {
    use super::*;
    use ratatui::style::Style;

    pub fn serialize<S: Serializer>(style: &Style, serializer: S) -> Result<S::Ok, S::Error> {
        crate::Style::from(*style).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Style, D::Error> {
        crate::Style::deserialize(deserializer).map(Style::from)
    }
}

/// A module for use via `#[serde(with)]` for serializing & deserializing
/// [`ratatui::style::Color`] values as color words and RGB codes.
///
/// Use it like so:
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Serialize)]
/// struct MyStruct {
///     #[serde(with = "parse_style::serde::ratatui::color")]
///     color: ratatui::style::Color,
/// }
/// ```
pub mod color {
    use super::*;
    use ratatui::style::Color;

    pub fn serialize<S: Serializer>(color: &Color, serializer: S) -> Result<S::Ok, S::Error> {
        crate::Color::from(*color).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Color, D::Error> {
        crate::Color::deserialize(deserializer).map(Color::from)
    }
}
