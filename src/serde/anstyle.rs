//! (De)serializing [`anstyle`] types
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};

/// A module for use via `#[serde(with)]` for serializing & deserializing
/// [`anstyle::Style`] values as style strings.
///
/// Use it like so:
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Serialize)]
/// struct MyStruct {
///     #[serde(with = "parse_style::serde::anstyle::style")]
///     style: anstyle::Style,
/// }
/// ```
pub mod style {
    use super::*;
    use anstyle::Style;

    pub fn serialize<S: Serializer>(style: &Style, serializer: S) -> Result<S::Ok, S::Error> {
        crate::Style::from(*style).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Style, D::Error> {
        crate::Style::deserialize(deserializer).map(Style::from)
    }
}

/// A module for use via `#[serde(with)]` for serializing & deserializing
/// [`anstyle::Color`] values as color words and RGB codes.
///
/// Use it like so:
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Serialize)]
/// struct MyStruct {
///     #[serde(with = "parse_style::serde::anstyle::color")]
///     color: anstyle::Color,
/// }
/// ```
///
/// Note that attempting to deserialize a string of the form `"default"` with
/// this module will produce an "invalid value" error.
pub mod color {
    use super::*;
    use anstyle::Color;
    use serde::de::Error;

    pub fn serialize<S: Serializer>(color: &Color, serializer: S) -> Result<S::Ok, S::Error> {
        crate::Color::from(*color).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Color, D::Error> {
        let c = crate::Color::deserialize(deserializer)?;
        Color::try_from(c).map_err(|_| D::Error::invalid_value(serde::de::Unexpected::Str("default"), &r##"a color word or a string of the form "color(INT)", "rgb(INT,INT,INT)", or "#xxxxxx""##))
    }
}

/// A module for use via `#[serde(with)]` for serializing & deserializing
/// [`anstyle::Ansi256Color`] values as color words.
///
/// Use it like so:
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Serialize)]
/// struct MyStruct {
///     #[serde(with = "parse_style::serde::anstyle::ansi256color")]
///     color: anstyle::Ansi256Color,
/// }
/// ```
pub mod ansi256color {
    use super::*;
    use anstyle::Ansi256Color;

    pub fn serialize<S: Serializer>(
        style: &Ansi256Color,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        crate::Color256::from(*style).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Ansi256Color, D::Error> {
        crate::Color256::deserialize(deserializer).map(Ansi256Color::from)
    }
}

/// A module for use via `#[serde(with)]` for serializing & deserializing
/// [`anstyle::RgbColor`] values as RGB codes.
///
/// Use it like so:
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Serialize)]
/// struct MyStruct {
///     #[serde(with = "parse_style::serde::anstyle::rgb_color")]
///     color: anstyle::RgbColor,
/// }
/// ```
pub mod rgb_color {
    use super::*;
    use anstyle::RgbColor;

    pub fn serialize<S: Serializer>(style: &RgbColor, serializer: S) -> Result<S::Ok, S::Error> {
        crate::RgbColor::from(*style).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<RgbColor, D::Error> {
        crate::RgbColor::deserialize(deserializer).map(RgbColor::from)
    }
}
