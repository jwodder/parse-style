use super::attributes::{Attribute, AttributeSet};
use super::color::Color;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Style {
    /// The foreground color
    foreground: Option<Color>,

    /// The background color
    background: Option<Color>,

    /// Active/enabled attributes
    enabled_attributes: AttributeSet,

    /// Explicitly disabled attributes
    disabled_attributes: AttributeSet,
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
        self
    }

    /// Set the disabled attributes
    pub fn disabled_attributes<A: Into<AttributeSet>>(mut self, attrs: A) -> Style {
        self.disabled_attributes = attrs.into();
        self
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

    /// Enable the given attribute
    pub fn enable_attribute(mut self, attr: Attribute) -> Style {
        self.enabled_attributes |= attr;
        self.disabled_attributes -= attr;
        self
    }

    /// Disable the given attribute
    pub fn disable_attribute(mut self, attr: Attribute) -> Style {
        self.enabled_attributes -= attr;
        self.disabled_attributes |= attr;
        self
    }

    /// Enable bold text
    pub fn bold(self) -> Style {
        self.enable_attribute(Attribute::Bold)
    }

    /// Enable dim text
    pub fn dim(self) -> Style {
        self.enable_attribute(Attribute::Dim)
    }

    /// Enable italic text
    pub fn italic(self) -> Style {
        self.enable_attribute(Attribute::Italic)
    }

    /// Enable underlining
    pub fn underline(self) -> Style {
        self.enable_attribute(Attribute::Underline)
    }

    /// Enable blinking
    pub fn blink(self) -> Style {
        self.enable_attribute(Attribute::Blink)
    }

    /// Enable fast blinking
    pub fn blink2(self) -> Style {
        self.enable_attribute(Attribute::Blink2)
    }

    /// Enable reverse video
    pub fn reverse(self) -> Style {
        self.enable_attribute(Attribute::Reverse)
    }

    /// Enable concealed/hidden text
    pub fn conceal(self) -> Style {
        self.enable_attribute(Attribute::Conceal)
    }

    /// Enable strikethrough
    pub fn strike(self) -> Style {
        self.enable_attribute(Attribute::Strike)
    }

    /// Enable double-underlining
    pub fn underline2(self) -> Style {
        self.enable_attribute(Attribute::Underline2)
    }

    /// Enable framed text
    pub fn frame(self) -> Style {
        self.enable_attribute(Attribute::Frame)
    }

    /// Enable encircled text
    pub fn encircle(self) -> Style {
        self.enable_attribute(Attribute::Encircle)
    }

    /// Enable overlining
    pub fn overline(self) -> Style {
        self.enable_attribute(Attribute::Overline)
    }

    /// Disable bold text
    pub fn no_bold(self) -> Style {
        self.disable_attribute(Attribute::Bold)
    }

    /// Disable dim text
    pub fn no_dim(self) -> Style {
        self.disable_attribute(Attribute::Dim)
    }

    /// Disable italic text
    pub fn no_italic(self) -> Style {
        self.disable_attribute(Attribute::Italic)
    }

    /// Disable underlining
    pub fn no_underline(self) -> Style {
        self.disable_attribute(Attribute::Underline)
    }

    /// Disable blinking
    pub fn no_blink(self) -> Style {
        self.disable_attribute(Attribute::Blink)
    }

    /// Disable fast blinking
    pub fn no_blink2(self) -> Style {
        self.disable_attribute(Attribute::Blink2)
    }

    /// Disable reverse video
    pub fn no_reverse(self) -> Style {
        self.disable_attribute(Attribute::Reverse)
    }

    /// Disable concealed/hidden text
    pub fn no_conceal(self) -> Style {
        self.disable_attribute(Attribute::Conceal)
    }

    /// Disable strikethrough
    pub fn no_strike(self) -> Style {
        self.disable_attribute(Attribute::Strike)
    }

    /// Disable double-underlining
    pub fn no_underline2(self) -> Style {
        self.disable_attribute(Attribute::Underline2)
    }

    /// Disable framed text
    pub fn no_frame(self) -> Style {
        self.disable_attribute(Attribute::Frame)
    }

    /// Disable encircled text
    pub fn no_encircle(self) -> Style {
        self.disable_attribute(Attribute::Encircle)
    }

    /// Disable overlining
    pub fn no_overline(self) -> Style {
        self.disable_attribute(Attribute::Overline)
    }
}
