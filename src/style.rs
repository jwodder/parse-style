use super::attributes::{Attribute, AttributeSet};
use super::color::Color;
use std::fmt;
use thiserror::Error;

/// A terminal text style
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Style {
    /// The foreground color
    pub foreground: Option<Color>,

    /// The background color
    pub background: Option<Color>,

    /// Active/enabled attributes
    pub enabled_attributes: AttributeSet,

    /// Explicitly disabled attributes
    ///
    /// Note that, while the individual `Style` methods will keep things
    /// "normalized," manual field manipulation can produce styles in which the
    /// same attribute is in both `enabled_attributes` and
    /// `disabled_attributes`; methods like [`is_enabled()`][Style::is_enabled]
    /// will treat such attributes as neither enabled nor disabled.
    pub disabled_attributes: AttributeSet,
}

impl Style {
    /// Create a new, empty style
    pub const fn new() -> Style {
        Style {
            foreground: None,
            background: None,
            enabled_attributes: AttributeSet::empty(),
            disabled_attributes: AttributeSet::empty(),
        }
    }

    /// Set the foreground color
    pub const fn foreground(mut self, fg: Option<Color>) -> Style {
        self.foreground = fg;
        self
    }

    /// Set the background color
    pub const fn background(mut self, bg: Option<Color>) -> Style {
        self.background = bg;
        self
    }

    /// Set the enabled attributes
    pub fn enabled_attributes<A: Into<AttributeSet>>(mut self, attrs: A) -> Style {
        self.enabled_attributes = attrs.into();
        self.disabled_attributes -= self.enabled_attributes;
        self
    }

    /// Set the disabled attributes
    pub fn disabled_attributes<A: Into<AttributeSet>>(mut self, attrs: A) -> Style {
        self.disabled_attributes = attrs.into();
        self.enabled_attributes -= self.disabled_attributes;
        self
    }

    /// `true` if the style does not set any colors or attributes
    pub fn is_empty(self) -> bool {
        self.foreground.is_none()
            && self.background.is_none()
            && self.enabled_attributes.is_empty()
            && self.disabled_attributes.is_empty()
    }

    /// `true` if `attr` is enabled and not disabled
    pub fn is_enabled(self, attr: Attribute) -> bool {
        self.enabled_attributes.contains(attr) && !self.disabled_attributes.contains(attr)
    }

    /// `true` if `attr` is disabled and not enabled
    pub fn is_disabled(self, attr: Attribute) -> bool {
        self.disabled_attributes.contains(attr) && !self.enabled_attributes.contains(attr)
    }

    /// Return the foreground color
    pub const fn get_foreground(self) -> Option<Color> {
        self.foreground
    }

    /// Return the background color
    pub const fn get_background(self) -> Option<Color> {
        self.background
    }

    /// Return the enabled attributes
    pub const fn get_enabled_attributes(self) -> AttributeSet {
        self.enabled_attributes
    }

    /// Return the disabled attributes
    pub const fn get_disabled_attributes(self) -> AttributeSet {
        self.disabled_attributes
    }

    /// Combine two styles, applying the effects of `other` after `self`
    pub fn patch(self, other: Style) -> Style {
        let foreground = self.foreground.or(other.foreground);
        let background = self.background.or(other.background);
        let enabled_attributes =
            (self.enabled_attributes - other.disabled_attributes) | other.enabled_attributes;
        let disabled_attributes =
            (self.disabled_attributes - other.enabled_attributes) | other.disabled_attributes;
        Style {
            foreground,
            background,
            enabled_attributes,
            disabled_attributes,
        }
    }

    /// Enable the given attribute(s)
    pub fn enable<A: Into<AttributeSet>>(mut self, attrs: A) -> Style {
        let attrs = attrs.into();
        self.enabled_attributes |= attrs;
        self.disabled_attributes -= attrs;
        self
    }

    /// Disable the given attribute(s)
    pub fn disable<A: Into<AttributeSet>>(mut self, attrs: A) -> Style {
        let attrs = attrs.into();
        self.enabled_attributes -= attrs;
        self.disabled_attributes |= attrs;
        self
    }

    /// Enable bold text
    pub fn bold(self) -> Style {
        self.enable(Attribute::Bold)
    }

    /// Enable dim text
    pub fn dim(self) -> Style {
        self.enable(Attribute::Dim)
    }

    /// Enable italic text
    pub fn italic(self) -> Style {
        self.enable(Attribute::Italic)
    }

    /// Enable underlining
    pub fn underline(self) -> Style {
        self.enable(Attribute::Underline)
    }

    /// Enable blinking
    pub fn blink(self) -> Style {
        self.enable(Attribute::Blink)
    }

    /// Enable fast blinking
    pub fn blink2(self) -> Style {
        self.enable(Attribute::Blink2)
    }

    /// Enable reverse video
    pub fn reverse(self) -> Style {
        self.enable(Attribute::Reverse)
    }

    /// Enable concealed/hidden text
    pub fn conceal(self) -> Style {
        self.enable(Attribute::Conceal)
    }

    /// Enable strikethrough
    pub fn strike(self) -> Style {
        self.enable(Attribute::Strike)
    }

    /// Enable double-underlining
    pub fn underline2(self) -> Style {
        self.enable(Attribute::Underline2)
    }

    /// Enable framed text
    pub fn frame(self) -> Style {
        self.enable(Attribute::Frame)
    }

    /// Enable encircled text
    pub fn encircle(self) -> Style {
        self.enable(Attribute::Encircle)
    }

    /// Enable overlining
    pub fn overline(self) -> Style {
        self.enable(Attribute::Overline)
    }

    /// Disable bold text
    pub fn not_bold(self) -> Style {
        self.disable(Attribute::Bold)
    }

    /// Disable dim text
    pub fn not_dim(self) -> Style {
        self.disable(Attribute::Dim)
    }

    /// Disable italic text
    pub fn not_italic(self) -> Style {
        self.disable(Attribute::Italic)
    }

    /// Disable underlining
    pub fn not_underline(self) -> Style {
        self.disable(Attribute::Underline)
    }

    /// Disable blinking
    pub fn not_blink(self) -> Style {
        self.disable(Attribute::Blink)
    }

    /// Disable fast blinking
    pub fn not_blink2(self) -> Style {
        self.disable(Attribute::Blink2)
    }

    /// Disable reverse video
    pub fn not_reverse(self) -> Style {
        self.disable(Attribute::Reverse)
    }

    /// Disable concealed/hidden text
    pub fn not_conceal(self) -> Style {
        self.disable(Attribute::Conceal)
    }

    /// Disable strikethrough
    pub fn not_strike(self) -> Style {
        self.disable(Attribute::Strike)
    }

    /// Disable double-underlining
    pub fn not_underline2(self) -> Style {
        self.disable(Attribute::Underline2)
    }

    /// Disable framed text
    pub fn not_frame(self) -> Style {
        self.disable(Attribute::Frame)
    }

    /// Disable encircled text
    pub fn not_encircle(self) -> Style {
        self.disable(Attribute::Encircle)
    }

    /// Disable overlining
    pub fn not_overline(self) -> Style {
        self.disable(Attribute::Overline)
    }
}

impl<C: Into<Color>> From<C> for Style {
    /// Construct a new `Style` using the given color as the foreground color
    fn from(value: C) -> Style {
        Style::new().foreground(Some(value.into()))
    }
}

impl From<Attribute> for Style {
    /// Construct a new `Style` that enables the given attribute
    fn from(value: Attribute) -> Style {
        Style::new().enable(value)
    }
}

impl From<AttributeSet> for Style {
    /// Construct a new `Style` that enables the given attributes
    fn from(value: AttributeSet) -> Style {
        Style::new().enabled_attributes(value)
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for attr in [
            Attribute::Bold,
            Attribute::Dim,
            Attribute::Italic,
            Attribute::Underline,
            Attribute::Blink,
            Attribute::Blink2,
            Attribute::Reverse,
            Attribute::Conceal,
            Attribute::Strike,
            Attribute::Underline2,
            Attribute::Frame,
            Attribute::Encircle,
            Attribute::Overline,
        ] {
            if self.is_enabled(attr) {
                if !std::mem::replace(&mut first, false) {
                    write!(f, " ")?;
                }
                write!(f, "{attr}")?;
            } else if self.is_disabled(attr) {
                if !std::mem::replace(&mut first, false) {
                    write!(f, " ")?;
                }
                write!(f, "not {attr}")?;
            }
        }
        if let Some(fg) = self.foreground {
            if !std::mem::replace(&mut first, false) {
                write!(f, " ")?;
            }
            write!(f, "{fg}")?;
        }
        if let Some(bg) = self.background {
            if !std::mem::replace(&mut first, false) {
                write!(f, " ")?;
            }
            write!(f, "on {bg}")?;
        }
        if first {
            write!(f, "none")?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Style {
    type Err = ParseStyleError;

    fn from_str(s: &str) -> Result<Style, ParseStyleError> {
        let mut style = Style::new();
        if s.is_empty() || s.trim().eq_ignore_ascii_case("none") {
            return Ok(style);
        }
        let mut words = s.split_whitespace();
        while let Some(token) = words.next() {
            if token.eq_ignore_ascii_case("on") {
                let Some(bg) = words.next().and_then(|s| s.parse::<Color>().ok()) else {
                    return Err(ParseStyleError::MissingBackground);
                };
                style.background = Some(bg);
            } else if token.eq_ignore_ascii_case("not") {
                let Some(attr) = words.next().and_then(|s| s.parse::<Attribute>().ok()) else {
                    return Err(ParseStyleError::MissingAttribute);
                };
                style = style.disable(attr);
            } else if let Ok(color) = token.parse::<Color>() {
                style.foreground = Some(color);
            } else if let Ok(attr) = token.parse::<Attribute>() {
                style = style.enable(attr);
            } else {
                return Err(ParseStyleError::Token(token.to_owned()));
            }
        }
        Ok(style)
    }
}

/// Error returned when parsing a style fails
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ParseStyleError {
    /// An invalid/unexpected token was enountered
    #[error("unexpected token in style string: {0:?}")]
    Token(
        /// The invalid token
        String,
    ),

    /// `"on"` was not followed by a valid color word
    #[error(r#""on" not followed by valid color word"#)]
    MissingBackground,

    /// `"not"` was not followed by a valid attribute name
    #[error(r#""not" not followed by valid attribute name"#)]
    MissingAttribute,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_is_default() {
        assert_eq!(Style::new(), Style::default());
    }

    mod display {
        use super::*;
        use crate::Color256;

        #[test]
        fn none() {
            assert_eq!(Style::new().to_string(), "none");
        }

        #[test]
        fn fg_color() {
            let style = Style::from(Color256::RED);
            assert_eq!(style.to_string(), "red");
        }

        #[test]
        fn bg_color() {
            let style = Color256::RED.as_background();
            assert_eq!(style.to_string(), "on red");
        }

        #[test]
        fn fg_on_bg() {
            let style = Color256::BLUE.on(Color256::RED);
            assert_eq!(style.to_string(), "blue on red");
        }

        #[test]
        fn attr() {
            let style = Style::from(Attribute::Bold);
            assert_eq!(style.to_string(), "bold");
        }

        #[test]
        fn multiple_attrs() {
            let style = Style::from(Attribute::Bold | Attribute::Reverse);
            assert_eq!(style.to_string(), "bold reverse");
        }

        #[test]
        fn not_attr() {
            let style = Style::new().disable(Attribute::Bold);
            assert_eq!(style.to_string(), "not bold");
        }

        #[test]
        fn multiple_not_attrs() {
            let style = Style::new()
                .disable(Attribute::Bold)
                .disable(Attribute::Reverse);
            assert_eq!(style.to_string(), "not bold not reverse");
        }

        #[test]
        fn attr_and_not_attr() {
            let style = Style::from(Attribute::Bold).disable(Attribute::Blink);
            assert_eq!(style.to_string(), "bold not blink");
        }

        #[test]
        fn gamut() {
            let style = Color256::YELLOW
                .on(Color::Default)
                .enable(Attribute::Italic)
                .disable(Attribute::Bold);
            assert_eq!(style.to_string(), "not bold italic yellow on default");
        }

        #[test]
        fn all_attrs() {
            let style = Style::from(AttributeSet::full());
            assert_eq!(style.to_string(), "bold dim italic underline blink blink2 reverse conceal strike underline2 frame encircle overline");
        }

        #[test]
        fn not_all_attrs() {
            let style = Style::new().disabled_attributes(AttributeSet::full());
            assert_eq!(style.to_string(), "not bold not dim not italic not underline not blink not blink2 not reverse not conceal not strike not underline2 not frame not encircle not overline");
        }
    }

    mod parse {
        use super::*;
        use crate::Color256;
        use rstest::rstest;

        #[test]
        fn none() {
            assert_eq!("".parse::<Style>().unwrap(), Style::new());
            assert_eq!("none".parse::<Style>().unwrap(), Style::new());
            assert_eq!("NONE".parse::<Style>().unwrap(), Style::new());
            assert_eq!(" none ".parse::<Style>().unwrap(), Style::new());
        }

        #[test]
        fn fg() {
            assert_eq!(
                "green".parse::<Style>().unwrap(),
                Style::from(Color256::GREEN)
            );
        }

        #[test]
        fn bg() {
            assert_eq!(
                "on green".parse::<Style>().unwrap(),
                Color256::GREEN.as_background()
            );
            assert_eq!(
                " on  green ".parse::<Style>().unwrap(),
                Color256::GREEN.as_background()
            );
            assert_eq!(
                " ON  GREEN ".parse::<Style>().unwrap(),
                Color256::GREEN.as_background()
            );
        }

        #[test]
        fn fg_on_bg() {
            assert_eq!(
                "blue on white".parse::<Style>().unwrap(),
                Color256::BLUE.on(Color256::WHITE)
            );
            assert_eq!(
                "on white blue".parse::<Style>().unwrap(),
                Color256::BLUE.on(Color256::WHITE)
            );
        }

        #[test]
        fn attr() {
            assert_eq!(
                "bold".parse::<Style>().unwrap(),
                Style::from(Attribute::Bold)
            );
        }

        #[test]
        fn multiple_attr() {
            assert_eq!(
                "bold underline".parse::<Style>().unwrap(),
                Style::from(Attribute::Bold | Attribute::Underline)
            );
            assert_eq!(
                "underline bold".parse::<Style>().unwrap(),
                Style::from(Attribute::Bold | Attribute::Underline)
            );
        }

        #[test]
        fn not_attr() {
            assert_eq!(
                "not bold".parse::<Style>().unwrap(),
                Style::new().disable(Attribute::Bold)
            );
            assert_eq!(
                " NOT  BOLD ".parse::<Style>().unwrap(),
                Style::new().disable(Attribute::Bold)
            );
        }

        #[test]
        fn multiple_not_attrs() {
            assert_eq!(
                "not bold not s".parse::<Style>().unwrap(),
                Style::new().disabled_attributes(Attribute::Bold | Attribute::Strike)
            );
            assert_eq!(
                "not s not bold".parse::<Style>().unwrap(),
                Style::new().disabled_attributes(Attribute::Bold | Attribute::Strike)
            );
        }

        #[test]
        fn attr_and_not_attr() {
            assert_eq!(
                "dim not blink2".parse::<Style>().unwrap(),
                Style::new()
                    .enable(Attribute::Dim)
                    .disable(Attribute::Blink2)
            );
            assert_eq!(
                "not blink2 dim".parse::<Style>().unwrap(),
                Style::new()
                    .enable(Attribute::Dim)
                    .disable(Attribute::Blink2)
            );
        }

        #[test]
        fn gamut() {
            for s in [
                "bold not underline red on blue",
                "not underline red on blue bold",
                "on blue red not underline bold",
            ] {
                assert_eq!(
                    s.parse::<Style>().unwrap(),
                    Color256::RED.on(Color256::BLUE).bold().not_underline()
                );
            }
        }

        #[test]
        fn multiple_fg() {
            assert_eq!(
                "red blue".parse::<Style>().unwrap(),
                Style::from(Color256::BLUE)
            );
        }

        #[test]
        fn multiple_bg() {
            assert_eq!(
                "on red on blue".parse::<Style>().unwrap(),
                Color256::BLUE.as_background()
            );
        }

        #[test]
        fn attr_on_and_off() {
            assert_eq!(
                "bold magenta not bold".parse::<Style>().unwrap(),
                Style::from(Color256::MAGENTA).not_bold()
            );
        }

        #[test]
        fn attr_off_and_on() {
            assert_eq!(
                "not bold magenta bold".parse::<Style>().unwrap(),
                Style::from(Color256::MAGENTA).bold()
            );
        }

        #[rstest]
        #[case("on bold")]
        #[case("on foo")]
        #[case("blue on")]
        #[case("on")]
        #[case("not blue")]
        #[case("not foo")]
        #[case("bold not")]
        #[case("not not bold italic")]
        #[case("not")]
        #[case("none red")]
        #[case("red none")]
        #[case("foo")]
        #[case("rgb(1, 2, 3)")]
        #[case("bright blue")]
        fn err(#[case] s: &str) {
            assert!(s.parse::<Style>().is_err());
        }
    }
}
