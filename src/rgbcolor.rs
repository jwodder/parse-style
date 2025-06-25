use super::ParseColorError;
use crate::color::Color;
use crate::style::Style;
use crate::util::strip_nocase_prefix;
use std::fmt;

/// A 24-bit color composed of red, green, and blue components
///
/// An `RgbColor` value can be [parsed][std::str::FromStr] from a string
/// consisting of `'#'` followed by six hexadecimal digits or from a string
/// of the form `"rgb({red},{blue},{green})"` where the individual components
/// are decimal integers from 0 through 255.
///
/// `RgbColor` values are [displayed][std::fmt::Display] as strings consisting
/// of `'#'` followed by six lowercase hexadecimal digits.
///
/// # Examples
///
/// ```
/// use parse_style::RgbColor;
///
/// assert_eq!("#e99695".parse::<RgbColor>().unwrap(), RgbColor(233, 150, 149));
/// assert_eq!("rgb(233,150,149)".parse::<RgbColor>().unwrap(), RgbColor(233, 150, 149));
///
/// assert_eq!(RgbColor(233, 150, 149).to_string(), "#e99695");
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RgbColor(
    /// Red
    pub u8,
    /// Green
    pub u8,
    /// Blue
    pub u8,
);

impl RgbColor {
    /// Return the red component
    pub fn red(self) -> u8 {
        self.0
    }

    /// Return the green component
    pub fn green(self) -> u8 {
        self.1
    }

    /// Return the blue component
    pub fn blue(self) -> u8 {
        self.2
    }

    /// Return a new [`Style`] that uses this color as the foreground color
    pub fn as_foreground(self) -> Style {
        Style::new().foreground(Some(self.into()))
    }

    /// Return a new [`Style`] that uses this color as the background color
    pub fn as_background(self) -> Style {
        Style::new().background(Some(self.into()))
    }

    /// Return a new [`Style`] that uses this color as the foreground color and
    /// `bg` as the background color
    pub fn on<C: Into<Color>>(self, bg: C) -> Style {
        Style::new()
            .foreground(Some(self.into()))
            .background(Some(bg.into()))
    }
}

impl From<(u8, u8, u8)> for RgbColor {
    fn from(value: (u8, u8, u8)) -> RgbColor {
        RgbColor(value.0, value.1, value.2)
    }
}

impl From<RgbColor> for (u8, u8, u8) {
    fn from(value: RgbColor) -> (u8, u8, u8) {
        (value.0, value.1, value.2)
    }
}

#[cfg(feature = "anstyle")]
#[cfg_attr(docsrs, doc(cfg(feature = "anstyle")))]
impl From<RgbColor> for anstyle::RgbColor {
    /// Convert an `RgbColor` to an [`anstyle::RgbColor`]
    fn from(value: RgbColor) -> anstyle::RgbColor {
        anstyle::RgbColor(value.0, value.1, value.2)
    }
}

#[cfg(feature = "anstyle")]
#[cfg_attr(docsrs, doc(cfg(feature = "anstyle")))]
impl From<anstyle::RgbColor> for RgbColor {
    /// Convert an [`anstyle::RgbColor`] to a `RgbColor`
    fn from(value: anstyle::RgbColor) -> RgbColor {
        RgbColor(value.0, value.1, value.2)
    }
}

#[cfg(feature = "crossterm")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossterm")))]
impl From<RgbColor> for crossterm::style::Color {
    /// Convert an `RgbColor` to a [`crossterm::style::Color`]
    fn from(value: RgbColor) -> crossterm::style::Color {
        crossterm::style::Color::Rgb {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}

#[cfg(feature = "ratatui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ratatui")))]
impl From<RgbColor> for ratatui::style::Color {
    /// Convert an `RgbColor` to a [`ratatui::style::Color`]
    fn from(value: RgbColor) -> ratatui::style::Color {
        ratatui::style::Color::Rgb(value.0, value.1, value.2)
    }
}

impl fmt::Display for RgbColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
}

impl std::str::FromStr for RgbColor {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<RgbColor, ParseColorError> {
        if let Some(hex) = s
            .strip_prefix('#')
            .filter(|s| s.chars().all(|c| c.is_ascii_hexdigit()) && s.len() == 6)
        {
            let red = u8::from_str_radix(&hex[..2], 16).expect("should be valid hex string");
            let green = u8::from_str_radix(&hex[2..4], 16).expect("should be valid hex string");
            let blue = u8::from_str_radix(&hex[4..], 16).expect("should be valid hex string");
            Ok(RgbColor(red, green, blue))
        } else if let Some(dec) = strip_nocase_prefix(s, "rgb(").and_then(|s| s.strip_suffix(')')) {
            let mut rgb = dec.split(',').map(str::parse::<u8>);
            let red = rgb.next();
            let green = rgb.next();
            let blue = rgb.next();
            let rest = rgb.next();
            if let (Some(Ok(red)), Some(Ok(green)), Some(Ok(blue)), None) = (red, green, blue, rest)
            {
                Ok(RgbColor(red, green, blue))
            } else {
                Err(ParseColorError(s.to_owned()))
            }
        } else {
            Err(ParseColorError(s.to_owned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_display() {
        assert_eq!(RgbColor(0x7F, 0xFF, 0x00).to_string(), "#7fff00");
    }

    #[rstest]
    #[case("#7fff00", RgbColor(0x7F, 0xFF, 0x00))]
    #[case("#7FFF00", RgbColor(0x7F, 0xFF, 0x00))]
    #[case("rgb(78,126,70)", RgbColor(78, 126, 70))]
    #[case("RGB(78,126,70)", RgbColor(78, 126, 70))]
    fn test_parse(#[case] s: &str, #[case] color: RgbColor) {
        assert_eq!(s.parse::<RgbColor>().unwrap(), color);
    }

    #[rstest]
    #[case("7fff00")]
    #[case("# 7fff00")]
    #[case("#000")]
    #[case("rgb(78, 126, 70)")]
    #[case("rgb(78,126)")]
    #[case("rgb(78,126,70,0)")]
    #[case("rgb(0x7f,0xff,0x00)")]
    fn test_parse_err(#[case] s: &str) {
        assert!(s.parse::<RgbColor>().is_err());
    }
}
