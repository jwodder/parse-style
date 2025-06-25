//! (De)serializing foreign types as style strings & color words
//!
//! This module contains `#[serde(with)]`-compatible submodules that make it
//! possible to directly serialize & deserialize types from foreign styling
//! crates using style string syntax (for style types) or color words/RGB codes
//! (for color types).
//!
//! Keep in mind that, due to incompatibilities between `parse-style`'s styling
//! concepts and those of other crates, some data may be lost when
//! (de)serializing using these modules.  For example, deserializing the string
//! `"red bold blink2"` as an `anstyle::Style` will result in a style with a red
//! foreground and only a bold effect, as anstyle does not support rapid
//! blinking.

#[cfg(feature = "anstyle")]
#[cfg_attr(docsrs, doc(cfg(all(feature = "anstyle", feature = "serde"))))]
pub mod anstyle;

#[cfg(feature = "crossterm")]
#[cfg_attr(docsrs, doc(cfg(all(feature = "crossterm", feature = "serde"))))]
pub mod crossterm;
