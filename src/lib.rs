#![cfg_attr(docsrs, feature(doc_cfg))]
//! `parse-style` is a [Rust](https://www.rust-lang.org) library for parsing &
//! displaying strings describing styles for terminal text using a syntax
//! compatible with the Python library
//! [`rich`](https://github.com/Textualize/rich).
//!
//! ```
//! use parse_style::{Color256, Style};
//!
//! assert_eq!(
//!     "bold red on blue".parse::<Style>().unwrap(),
//!     Style::new()
//!         .foreground(Some(Color256::RED.into()))
//!         .background(Some(Color256::BLUE.into()))
//!         .bold()
//! );
//!
//! let style = Style::from(Color256::BRIGHT_GREEN).underline();
//! assert_eq!(style.to_string(), "underline bright_green");
//! ```
//!
//! Note that this library does not provide functionality for rendering styles
//! as ANSI escape sequences; there are plenty of crates that do that already,
//! and `parse-style` provides conversions to some of those crates' types so
//! you can use them for your actual styling.
//!
//! Style String Syntax
//! ===================
//!
//! `parse-style` follows `rich`'s [style string syntax specification][syntax],
//! specifically:
//!
//! - A style string may be the empty string or `"none"` (case insensitive),
//!   denoting an empty style, or it may be a space-separated sequence of one or
//!   more of the following tokens in any order:
//!
//!     - A color word (see below), denoting a foreground color
//!
//!     - The word "`on`" followed by a color word, denoting a background color
//!
//!     - One of the following attribute names (case insensitive), indicating that
//!       the given text attribute should be enabled:
//!         - "`bold`" or "`b`" — bold text
//!         - "`dim`" or "`d`" — dim/faint text
//!         - "`italic`" or "`i`" — italic text
//!         - "`underline`" or "`u`" — underlined text
//!         - "`blink`" — blinking text
//!         - "`blink2`" — fast/rapidly blinking text
//!         - "`reverse`" or "`r`" — reverse-video text
//!         - "`conceal`" or "`c`" — concealed/hidden text
//!         - "`strike`" or "`s`" — struck-through text
//!         - "`underline2`" or "`uu`" — double-underlined text
//!         - "`frame`" — framed text
//!         - "`encircle`" — encircled text
//!         - "`overline`" — overlined text
//!
//!     - The word "`not`" followed by one of the above attribute names, indicating
//!       that the given text attribute should be disabled
//!
//! - Colors may be specified as any of the following:
//!
//!     - "`default`" (case insensitive), denoting the default terminal foreground
//!       or background color
//!
//!     - a color name (case insensitive) from [this table][colors]
//!
//!     - a word of the form `color({index})` (case insensitive) where
//!       `{index}` is a decimal integer from 0 through 255, denoting the 8-bit
//!       color with the given index
//!
//!     - a word of the form `#xxxxxx`, where the `x`'s are hexadecimal digits,
//!       denoting an RGB color
//!
//!     - a word of the form `rgb({red},{green},{blue})` (case insensitive) where
//!       `{red}`, `{green}`, and `{blue}` are decimal integers from 0 though 255,
//!       denoting an RGB color
//!
//! - If a style string contains two or more foreground colors, the last one is
//!   used, and likewise for background colors.
//!
//! - If a style string contains both an attribute and `not` the same attribute,
//!   the last occurrence wins.
//!
//! [syntax]: https://rich.readthedocs.io/en/stable/style.html
//! [colors]: https://rich.readthedocs.io/en/stable/appendix/colors.html
//!
//! Differences from `rich` Style Syntax
//! ------------------------------------
//!
//! - Hyperlink syntax (`"link https://www.example.com"`) is not supported.
//!
//! - Minor technical difference: `parse-style` uses Rust's definition of
//!   whitespace for splitting strings into tokens, while `rich` uses Python's
//!   space definition, which includes a few extra control characters.
//!
//! Features
//! ========
//!
//! The `parse-style` crate has the following optional features:
//!
//! - `anstyle` — Enables conversions between `parse-style` types and types
//!   from the [`anstyle`](https://crates.io/crates/anstyle) crate
//!
//! - `crossterm` — Enables conversions between `parse-style` types and types
//!   from the [`crossterm`](https://crates.io/crates/crossterm) crate
//!
//! - `ratatui` — Enables conversions between `parse-style` types and types
//!   from the [`ratatui`](https://crates.io/crates/ratatui) crate
//!
//! - `serde` — Enables [`serde`](https://serde.rs) implementations for
//!   (de)serializing `Style` values as style strings and colors as color
//!   strings.  When combined with one or more of the above features, also
//!   enables `#[serde(with)]`-compatible modules for (de)serializing foreign
//!   types in the same way.
//!
//! Important: Lossy Conversions
//! ============================
//!
//! Different terminal text-styling crates support different styling features,
//! making perfect interoperability impossible.  As a result, some conversions
//! between `parse-style` types and foreign types must discard some information
//! due to the target type being unable to represent it.  See the "Data Loss"
//! sections in the documentation of the `From` impls for specific information.

mod attributes;
mod color;
mod color256;
mod rgbcolor;
mod style;
mod util;
pub use crate::attributes::{
    Attribute, AttributeIter, AttributeSet, AttributeSetIter, ParseAttributeError,
};
pub use crate::color::Color;
pub use crate::color256::Color256;
pub use crate::rgbcolor::RgbColor;
pub use crate::style::{ParseStyleError, Style};
use thiserror::Error;

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde;

/// Error returned when parsing a color string fails
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("invalid color string: {0:?}")]
pub struct ParseColorError(
    /// The invalid color string
    pub String,
);

/// Error returned when conversion between a `parse_style` type and a foreign
/// type fails
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("failed to convert between parse_style type and foreign type")]
pub struct ConversionError;
