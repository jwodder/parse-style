use super::ParseColorError;
use crate::color256::Color256;
use crate::rgbcolor::RgbColor;
use crate::style::Style;
use std::fmt;

/// An enum of the different color types
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Color {
    /// The terminal's default foreground or background color
    #[default]
    Default,
    Color256(Color256),
    Rgb(RgbColor),
}

impl Color {
    /// Return a new [`Style`] that uses this color as the foreground color
    pub fn as_foreground(self) -> Style {
        Style::new().foreground(Some(self))
    }

    /// Return a new [`Style`] that uses this color as the background color
    pub fn as_background(self) -> Style {
        Style::new().background(Some(self))
    }

    /// Return a new [`Style`] that uses this color as the foreground color and
    /// `bg` as the background color
    pub fn on<C: Into<Color>>(self, bg: C) -> Style {
        Style::new()
            .foreground(Some(self))
            .background(Some(bg.into()))
    }
}

impl From<Color256> for Color {
    fn from(value: Color256) -> Color {
        Color::Color256(value)
    }
}

impl From<RgbColor> for Color {
    fn from(value: RgbColor) -> Color {
        Color::Rgb(value)
    }
}

impl From<u8> for Color {
    fn from(value: u8) -> Color {
        Color::Color256(Color256::from(value))
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Color {
        Color::Rgb(RgbColor::from(value))
    }
}

#[cfg(feature = "anstyle")]
#[cfg_attr(docsrs, doc(cfg(feature = "anstyle")))]
impl TryFrom<Color> for anstyle::Color {
    type Error = crate::ConversionError;

    /// Convert a `Color` to an [`anstyle::Color`]
    ///
    /// # Errors
    ///
    /// Returns `Err` if `value` is `Color::Default`, which is not
    /// representable by `anstyle::Color`
    fn try_from(value: Color) -> Result<anstyle::Color, crate::ConversionError> {
        match value {
            Color::Default => Err(crate::ConversionError),
            Color::Color256(c) => Ok(anstyle::Color::Ansi256(c.into())),
            Color::Rgb(c) => Ok(anstyle::Color::Rgb(c.into())),
        }
    }
}

#[cfg(feature = "anstyle")]
#[cfg_attr(docsrs, doc(cfg(feature = "anstyle")))]
impl From<anstyle::Color> for Color {
    /// Convert an [`anstyle::Color`] to a `Color`
    fn from(value: anstyle::Color) -> Color {
        match value {
            anstyle::Color::Ansi(c) => Color::Color256(c.into()),
            anstyle::Color::Ansi256(c) => Color::Color256(c.into()),
            anstyle::Color::Rgb(c) => Color::Rgb(c.into()),
        }
    }
}

#[cfg(feature = "crossterm")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossterm")))]
impl From<Color> for crossterm::style::Color {
    /// Convert a `Color` to a [`crossterm::style::Color`]
    fn from(value: Color) -> crossterm::style::Color {
        match value {
            Color::Default => crossterm::style::Color::Reset,
            Color::Color256(c) => c.into(),
            Color::Rgb(c) => c.into(),
        }
    }
}

#[cfg(feature = "crossterm")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossterm")))]
impl From<crossterm::style::Color> for Color {
    /// Convert a [`crossterm::style::Color`] to a `Color`
    fn from(value: crossterm::style::Color) -> Color {
        match value {
            crossterm::style::Color::Reset => Color::Default,
            crossterm::style::Color::Black => Color256::BRIGHT_BLACK.into(),
            crossterm::style::Color::DarkGrey => Color256::BLACK.into(),
            crossterm::style::Color::Red => Color256::BRIGHT_RED.into(),
            crossterm::style::Color::DarkRed => Color256::RED.into(),
            crossterm::style::Color::Green => Color256::BRIGHT_GREEN.into(),
            crossterm::style::Color::DarkGreen => Color256::GREEN.into(),
            crossterm::style::Color::Yellow => Color256::BRIGHT_YELLOW.into(),
            crossterm::style::Color::DarkYellow => Color256::YELLOW.into(),
            crossterm::style::Color::Blue => Color256::BRIGHT_BLUE.into(),
            crossterm::style::Color::DarkBlue => Color256::BLUE.into(),
            crossterm::style::Color::Magenta => Color256::BRIGHT_MAGENTA.into(),
            crossterm::style::Color::DarkMagenta => Color256::MAGENTA.into(),
            crossterm::style::Color::Cyan => Color256::BRIGHT_CYAN.into(),
            crossterm::style::Color::DarkCyan => Color256::CYAN.into(),
            crossterm::style::Color::White => Color256::BRIGHT_WHITE.into(),
            crossterm::style::Color::Grey => Color256::WHITE.into(),
            crossterm::style::Color::Rgb { r, g, b } => RgbColor(r, g, b).into(),
            crossterm::style::Color::AnsiValue(index) => Color256(index).into(),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Default => write!(f, "default"),
            Color::Color256(c) => write!(f, "{c}"),
            Color::Rgb(c) => write!(f, "{c}"),
        }
    }
}

impl std::str::FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Color, ParseColorError> {
        if s.eq_ignore_ascii_case("default") {
            Ok(Color::Default)
        } else {
            s.parse::<Color256>()
                .map(Color::from)
                .or_else(|_| s.parse::<RgbColor>().map(Color::from))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_default() {
        assert_eq!(Color::Default.to_string(), "default");
    }

    #[test]
    fn test_display_color256() {
        assert_eq!(Color::from(118).to_string(), "chartreuse1");
    }

    #[test]
    fn test_display_rgbcolor() {
        assert_eq!(Color::from((111, 120, 189)).to_string(), "#6f78bd");
    }

    #[test]
    fn test_parse_default() {
        assert_eq!("default".parse::<Color>().unwrap(), Color::Default);
    }

    #[test]
    fn test_parse_color256() {
        assert_eq!(
            "chartreuse1".parse::<Color>().unwrap(),
            Color::Color256(Color256(118))
        );
    }

    #[test]
    fn test_parse_rgbcolor() {
        assert_eq!(
            "#6f78bd".parse::<Color>().unwrap(),
            Color::Rgb(RgbColor(111, 120, 189))
        );
    }

    #[test]
    fn test_parse_err() {
        assert!("mauve".parse::<Color>().is_err());
    }
}
