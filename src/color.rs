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
