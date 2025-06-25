use flagset::{flags, FlagSet};
use std::fmt;
use thiserror::Error;

/// A set of [`Attribute`] values
pub type AttributeSet = FlagSet<Attribute>;

flags! {
    /// Individual effects that can be applied to text in a terminal.
    ///
    /// [`Attribute`] values can be combined with bitwise operators to produce
    /// [`AttributeSet`]s.
    ///
    /// `Attribute` values can be [parsed][std::str::FromStr] from the
    /// following case-insensitive strings:
    ///
    /// - `"bold"` or `"b"` — `Bold`
    /// - `"dim"` or `"d"` — `Dim`
    /// - `"italic"` or `"i"` — `Italic`
    /// - `"underline"` or `"u"` — `Underline`
    /// - `"blink"` — `Blink`
    /// - `"blink2"` — `Blink2`
    /// - `"reverse"` or `"r"` — `Reverse`
    /// - `"conceal"` or `"c"` — `Conceal`
    /// - `"strike"` or `"s"` — `Strike`
    /// - `"underline2"` or `"uu"` — `Underline2`
    /// - `"frame"` — `Frame`
    /// - `"encircle"` — `Encircle`
    /// - `"overline"` — `Overline`
    ///
    /// `Attribute` values are [displayed][std::fmt::Display] as lowercase
    /// strings from the above list; for values with two strings, the longer
    /// one is used.
    #[derive(Hash, Ord, PartialOrd)]
    pub enum Attribute: u16 {
        Bold,
        Dim,
        Italic,
        Underline,
        Blink,
        /// Fast blinking
        Blink2,
        /// Reverse video
        Reverse,
        /// Concealed/hidden
        Conceal,
        /// Strikethrough
        Strike,
        /// Double-underline
        Underline2,
        Frame,
        Encircle,
        Overline,
    }
}

impl Attribute {
    pub fn as_str(self) -> &'static str {
        match self {
            Attribute::Bold => "bold",
            Attribute::Dim => "dim",
            Attribute::Italic => "italic",
            Attribute::Underline => "underline",
            Attribute::Blink => "blink",
            Attribute::Blink2 => "blink2",
            Attribute::Reverse => "reverse",
            Attribute::Conceal => "conceal",
            Attribute::Strike => "strike",
            Attribute::Underline2 => "underline2",
            Attribute::Frame => "frame",
            Attribute::Encircle => "encircle",
            Attribute::Overline => "overline",
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Attribute {
    type Err = ParseAttributeError;

    fn from_str(s: &str) -> Result<Attribute, ParseAttributeError> {
        match s.to_ascii_lowercase().as_str() {
            "bold" | "b" => Ok(Attribute::Bold),
            "dim" | "d" => Ok(Attribute::Dim),
            "italic" | "i" => Ok(Attribute::Italic),
            "underline" | "u" => Ok(Attribute::Underline),
            "blink" => Ok(Attribute::Blink),
            "blink2" => Ok(Attribute::Blink2),
            "reverse" | "r" => Ok(Attribute::Reverse),
            "conceal" | "c" => Ok(Attribute::Conceal),
            "strike" | "s" => Ok(Attribute::Strike),
            "underline2" | "uu" => Ok(Attribute::Underline2),
            "frame" => Ok(Attribute::Frame),
            "encircle" => Ok(Attribute::Encircle),
            "overline" => Ok(Attribute::Overline),
            _ => Err(ParseAttributeError(s.to_owned())),
        }
    }
}

/// Error returned when parsing an attribute fails
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("invalid attribute name: {0:?}")]
pub struct ParseAttributeError(
    /// The invalid attribute string
    pub String,
);
