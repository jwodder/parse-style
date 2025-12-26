use super::attributes::{Attribute, AttributeSet};
use super::color::Color;
use std::fmt;
use thiserror::Error;

/// A terminal text style
///
/// `Style` stores two sets of [`Attribute`]s: those attributes that the style
/// enables, plus those that the style explicitly disables/turns off.  The
/// latter are relevant if you're applying a style in the middle of text with a
/// different style; e.g., if a text styled with `"red bold"` contains a
/// substring styled with `"blue not bold"`, you'd want to know that the bold
/// effect should be disabled for that substring.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Style {
    /// The foreground color
    foreground: Option<Color>,

    /// The background color
    background: Option<Color>,

    /// Active/enabled attributes
    enabled_attributes: AttributeSet,

    /// Explicitly disabled attributes
    ///
    /// The individual `Style` methods must ensure that no attribute is ever in
    /// both `enabled_attributes` and `disabled_attributes` at once.
    disabled_attributes: AttributeSet,
}

impl Style {
    /// Create a new, empty style
    pub const fn new() -> Style {
        Style {
            foreground: None,
            background: None,
            enabled_attributes: AttributeSet::EMPTY,
            disabled_attributes: AttributeSet::EMPTY,
        }
    }

    /// Set or clear the foreground color.
    ///
    /// Note that setting the foreground to `None` is different from setting it
    /// to [`Color::Default`]: if the style is applied in the middle of text
    /// with a foreground color set, `None` will leave the foreground color
    /// as-is while `Color::Default` will reset it.
    pub const fn foreground(mut self, fg: Option<Color>) -> Style {
        self.foreground = fg;
        self
    }

    /// Set or clear the background color
    ///
    /// Note that setting the background to `None` is different from setting it
    /// to [`Color::Default`]: if the style is applied in the middle of text
    /// with a background color set, `None` will leave the background color
    /// as-is while `Color::Default` will reset it.
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

    /// `true` if `attr` is enabled
    pub fn is_enabled(self, attr: Attribute) -> bool {
        self.enabled_attributes.contains(attr) && !self.disabled_attributes.contains(attr)
    }

    /// `true` if `attr` is explicitly disabled
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

#[cfg(feature = "anstyle")]
#[cfg_attr(docsrs, doc(cfg(feature = "anstyle")))]
impl From<Style> for anstyle::Style {
    /// Convert a `Style` to an [`anstyle::Style`]
    ///
    /// # Data Loss
    ///
    /// If the `Style`'s foreground or background color is [`Color::Default`],
    /// it will be converted to `None`.
    ///
    /// The following attributes will be discarded during conversion:
    ///
    /// - [`Attribute::Blink2`]
    /// - [`Attribute::Frame`]
    /// - [`Attribute::Encircle`]
    /// - [`Attribute::Overline`]
    ///
    /// Disabled attributes are discarded during conversion.
    fn from(value: Style) -> anstyle::Style {
        anstyle::Style::new()
            .fg_color(
                value
                    .get_foreground()
                    .and_then(|c| anstyle::Color::try_from(c).ok()),
            )
            .bg_color(
                value
                    .get_background()
                    .and_then(|c| anstyle::Color::try_from(c).ok()),
            )
            .effects(value.enabled_attributes.into())
    }
}

#[cfg(feature = "anstyle")]
#[cfg_attr(docsrs, doc(cfg(feature = "anstyle")))]
impl From<anstyle::Style> for Style {
    /// Convert an [`anstyle::Style`] to a `Style`
    ///
    /// # Data Loss
    ///
    /// Underline color is discarded during conversion.
    ///
    /// The following effects are discarded during conversion:
    ///
    /// - [`anstyle::Effects::CURLY_UNDERLINE`]
    /// - [`anstyle::Effects::DOTTED_UNDERLINE`]
    /// - [`anstyle::Effects::DASHED_UNDERLINE`]
    fn from(value: anstyle::Style) -> Style {
        Style::new()
            .foreground(value.get_fg_color().map(Color::from))
            .background(value.get_bg_color().map(Color::from))
            .enabled_attributes(AttributeSet::from(value.get_effects()))
    }
}

#[cfg(feature = "crossterm")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossterm")))]
impl From<crossterm::style::Attributes> for Style {
    /// Convert a [`crossterm::style::Attributes`] to a `Style`
    ///
    /// # Data Loss
    ///
    /// The following attributes are discarded during conversion:
    ///
    /// - [`crossterm::style::Attribute::Undercurled`]
    /// - [`crossterm::style::Attribute::Underdotted`]
    /// - [`crossterm::style::Attribute::Underdashed`]
    /// - [`crossterm::style::Attribute::Fraktur`]
    /// - [`crossterm::style::Attribute::NoBold`] (because it's dysfunctional)
    fn from(value: crossterm::style::Attributes) -> Style {
        use crossterm::style::Attribute as CrossAttrib;
        let mut set = Style::new();
        for attr in CrossAttrib::iterator().filter(|&attr| value.has(attr)) {
            match attr {
                CrossAttrib::Reset => set = Style::new(),
                CrossAttrib::Bold => set = set.bold(),
                CrossAttrib::Dim => set = set.dim(),
                CrossAttrib::Italic => set = set.italic(),
                CrossAttrib::Underlined => set = set.underline(),
                CrossAttrib::DoubleUnderlined => set = set.underline2(),
                CrossAttrib::Undercurled => (),
                CrossAttrib::Underdotted => (),
                CrossAttrib::Underdashed => (),
                CrossAttrib::SlowBlink => set = set.blink(),
                CrossAttrib::RapidBlink => set = set.blink2(),
                CrossAttrib::Reverse => set = set.reverse(),
                CrossAttrib::Hidden => set = set.conceal(),
                CrossAttrib::CrossedOut => set = set.strike(),
                CrossAttrib::Fraktur => (),
                CrossAttrib::NoBold => (),
                CrossAttrib::NormalIntensity => set = set.not_bold().not_dim(),
                CrossAttrib::NoItalic => set = set.not_italic(),
                CrossAttrib::NoUnderline => set = set.not_underline().not_underline2(),
                CrossAttrib::NoBlink => set = set.not_blink().not_blink2(),
                CrossAttrib::NoReverse => set = set.not_reverse(),
                CrossAttrib::NoHidden => set = set.not_conceal(),
                CrossAttrib::NotCrossedOut => set = set.not_strike(),
                CrossAttrib::Framed => set = set.frame(),
                CrossAttrib::Encircled => set = set.encircle(),
                CrossAttrib::OverLined => set = set.overline(),
                CrossAttrib::NotFramedOrEncircled => set = set.not_frame().not_encircle(),
                CrossAttrib::NotOverLined => set = set.not_overline(),
                _ => (), // non-exhaustive
            }
        }
        set
    }
}

#[cfg(feature = "crossterm")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossterm")))]
impl From<Style> for crossterm::style::ContentStyle {
    /// Convert a `Style` to a [`crossterm::style::ContentStyle`]
    ///
    /// # Data Loss
    ///
    /// Certain pairs of `parse-style` attributes are disabled by a single
    /// shared `crossterm` attribute.  Thus, when one element of a pair occurs
    /// in the disabled attributes, the resulting `ContentStyle` will disable
    /// both elements of the pair.
    ///
    /// The pairs are as follows:
    ///
    /// - [`Attribute::Bold`] and [`Attribute::Dim`] — both disabled by
    ///   [`crossterm::style::Attribute::NormalIntensity`]
    ///
    /// - [`Attribute::Blink`] and [`Attribute::Blink2`] — both disabled by
    ///   [`crossterm::style::Attribute::NoBlink`]
    ///
    /// - [`Attribute::Underline`] and [`Attribute::Underline2`] — both
    ///   disabled by [`crossterm::style::Attribute::NoUnderline`]
    ///
    /// - [`Attribute::Frame`] and [`Attribute::Encircle`] — both disabled by
    ///   [`crossterm::style::Attribute::NotFramedOrEncircled`]
    fn from(value: Style) -> crossterm::style::ContentStyle {
        use crossterm::style::Attribute as CrossAttrib;
        let foreground_color = value.foreground.map(crossterm::style::Color::from);
        let background_color = value.background.map(crossterm::style::Color::from);
        let mut attributes = crossterm::style::Attributes::from(value.enabled_attributes);
        for attr in value.disabled_attributes {
            match attr {
                Attribute::Bold => attributes.set(CrossAttrib::NormalIntensity),
                Attribute::Dim => attributes.set(CrossAttrib::NormalIntensity),
                Attribute::Italic => attributes.set(CrossAttrib::NoItalic),
                Attribute::Underline => attributes.set(CrossAttrib::NoUnderline),
                Attribute::Blink => attributes.set(CrossAttrib::NoBlink),
                Attribute::Blink2 => attributes.set(CrossAttrib::NoBlink),
                Attribute::Reverse => attributes.set(CrossAttrib::NoReverse),
                Attribute::Conceal => attributes.set(CrossAttrib::NoHidden),
                Attribute::Strike => attributes.set(CrossAttrib::NotCrossedOut),
                Attribute::Underline2 => attributes.set(CrossAttrib::NoUnderline),
                Attribute::Frame => attributes.set(CrossAttrib::NotFramedOrEncircled),
                Attribute::Encircle => attributes.set(CrossAttrib::NotFramedOrEncircled),
                Attribute::Overline => attributes.set(CrossAttrib::NotOverLined),
            }
        }
        crossterm::style::ContentStyle {
            foreground_color,
            background_color,
            attributes,
            underline_color: None,
        }
    }
}

#[cfg(feature = "crossterm")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossterm")))]
impl From<crossterm::style::ContentStyle> for Style {
    /// Convert a [`crossterm::style::ContentStyle`] to a `Style`
    ///
    /// # Data Loss
    ///
    /// Underline color is discarded during conversion.
    ///
    /// The following attributes are discarded during conversion:
    ///
    /// - [`crossterm::style::Attribute::Undercurled`]
    /// - [`crossterm::style::Attribute::Underdotted`]
    /// - [`crossterm::style::Attribute::Underdashed`]
    /// - [`crossterm::style::Attribute::Fraktur`]
    fn from(value: crossterm::style::ContentStyle) -> Style {
        Style::from(value.attributes)
            .foreground(value.foreground_color.map(Color::from))
            .background(value.background_color.map(Color::from))
    }
}

#[cfg(feature = "ratatui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ratatui")))]
impl From<Style> for ratatui_core::style::Style {
    /// Convert a `Style` to a [`ratatui_core::style::Style`]
    ///
    /// # Data Loss
    ///
    /// The following attributes are discarded during conversion:
    ///
    /// - [`Attribute::Underline2`]
    /// - [`Attribute::Frame`]
    /// - [`Attribute::Encircle`]
    /// - [`Attribute::Overline`]
    fn from(value: Style) -> ratatui_core::style::Style {
        // Don't try to construct a ratatui Style using struct notation, as the
        // `underline_color` field is feature-based.
        let mut style = ratatui_core::style::Style::new();
        if let Some(fg) = value.foreground.map(ratatui_core::style::Color::from) {
            style = style.fg(fg);
        }
        if let Some(bg) = value.background.map(ratatui_core::style::Color::from) {
            style = style.bg(bg);
        }
        style = style.add_modifier(value.enabled_attributes.into());
        style = style.remove_modifier(value.disabled_attributes.into());
        style
    }
}

#[cfg(feature = "ratatui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ratatui")))]
impl From<ratatui_core::style::Style> for Style {
    /// Convert a [`ratatui_core::style::Style`] to a `Style`
    ///
    /// # Data Loss
    ///
    /// Underline color is discarded during conversion.
    fn from(value: ratatui_core::style::Style) -> Style {
        let foreground = value.fg.map(Color::from);
        let background = value.bg.map(Color::from);
        let enabled_attributes = AttributeSet::from(value.add_modifier);
        let disabled_attributes = AttributeSet::from(value.sub_modifier);
        Style {
            foreground,
            background,
            enabled_attributes,
            disabled_attributes,
        }
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for attr in Attribute::iter() {
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

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl serde::Serialize for Style {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> serde::Deserialize<'de> for Style {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = Style;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a style string")
            }

            fn visit_str<E>(self, input: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                input
                    .parse::<Style>()
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::Str(input), &self))
            }
        }

        deserializer.deserialize_str(Visitor)
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
            let style = Style::from(AttributeSet::ALL);
            assert_eq!(
                style.to_string(),
                "bold dim italic underline blink blink2 reverse conceal strike underline2 frame encircle overline"
            );
        }

        #[test]
        fn not_all_attrs() {
            let style = Style::new().disabled_attributes(AttributeSet::ALL);
            assert_eq!(
                style.to_string(),
                "not bold not dim not italic not underline not blink not blink2 not reverse not conceal not strike not underline2 not frame not encircle not overline"
            );
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
