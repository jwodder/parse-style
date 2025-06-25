use std::fmt;
use thiserror::Error;

/// Individual effects that can be applied to text in a terminal.
///
/// `Attribute` values can be combined with bitwise operators to produce
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
#[derive(Clone, Copy, Debug, strum::EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum Attribute {
    Bold = 1 << 0,
    Dim = 1 << 1,
    Italic = 1 << 2,
    Underline = 1 << 3,
    Blink = 1 << 4,
    /// Fast blinking
    Blink2 = 1 << 5,
    /// Reverse video
    Reverse = 1 << 6,
    /// Concealed/hidden
    Conceal = 1 << 7,
    /// Strikethrough
    Strike = 1 << 8,
    /// Double-underline
    Underline2 = 1 << 9,
    Frame = 1 << 10,
    Encircle = 1 << 11,
    Overline = 1 << 12,
}

impl Attribute {
    const COUNT: u16 = 13;

    /// Returns an iterator over all [`Attribute`] variants
    pub fn iter() -> AttributeIter {
        // To avoid the need for users to import the trait
        <Attribute as strum::IntoEnumIterator>::iter()
    }

    /// Return the long name of the attribute
    ///
    /// # Example
    ///
    /// ```
    /// use parse_style::Attribute;
    ///
    /// assert_eq!(Attribute::Bold.as_str(), "bold");
    /// ```
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

impl<A: Into<AttributeSet>> std::ops::BitAnd<A> for Attribute {
    type Output = AttributeSet;

    fn bitand(self, rhs: A) -> AttributeSet {
        AttributeSet((self as u16) & rhs.into().0)
    }
}

impl<A: Into<AttributeSet>> std::ops::BitOr<A> for Attribute {
    type Output = AttributeSet;

    fn bitor(self, rhs: A) -> AttributeSet {
        AttributeSet((self as u16) | rhs.into().0)
    }
}

impl<A: Into<AttributeSet>> std::ops::BitXor<A> for Attribute {
    type Output = AttributeSet;

    fn bitxor(self, rhs: A) -> AttributeSet {
        AttributeSet((self as u16) ^ rhs.into().0)
    }
}

impl<A: Into<AttributeSet>> std::ops::Sub<A> for Attribute {
    type Output = AttributeSet;

    fn sub(self, rhs: A) -> AttributeSet {
        AttributeSet((self as u16) & !rhs.into().0)
    }
}

impl std::ops::Not for Attribute {
    type Output = AttributeSet;

    fn not(self) -> AttributeSet {
        AttributeSet::ALL - self
    }
}

/// A set of [`Attribute`] values.
///
/// `AttributeSet` values can be combined with bitwise operators and can be
/// iterated over.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AttributeSet(u16);

impl AttributeSet {
    /// A set containing no [`Attribute`]s
    pub const EMPTY: AttributeSet = AttributeSet(0);

    /// A set containing all [`Attribute`]s
    pub const ALL: AttributeSet = AttributeSet((1 << Attribute::COUNT) - 1);

    /// Return a new set containing no [`Attribute`]s
    pub fn new() -> AttributeSet {
        AttributeSet(0)
    }

    /// Test whether the set is empty
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Test whether the set contains all [`Attribute`]s
    pub fn is_all(self) -> bool {
        self == Self::ALL
    }

    /// Test whether the set contains the given [`Attribute`]
    pub fn contains(self, attr: Attribute) -> bool {
        self.0 & (attr as u16) != 0
    }
}

impl From<Attribute> for AttributeSet {
    fn from(value: Attribute) -> AttributeSet {
        AttributeSet(value as u16)
    }
}

impl IntoIterator for AttributeSet {
    type Item = Attribute;
    type IntoIter = AttributeSetIter;

    fn into_iter(self) -> AttributeSetIter {
        AttributeSetIter::new(self)
    }
}

impl Extend<Attribute> for AttributeSet {
    fn extend<I: IntoIterator<Item = Attribute>>(&mut self, iter: I) {
        for attr in iter {
            *self |= attr;
        }
    }
}

impl<A: Into<AttributeSet>> std::ops::BitAnd<A> for AttributeSet {
    type Output = AttributeSet;

    fn bitand(self, rhs: A) -> AttributeSet {
        AttributeSet(self.0 & rhs.into().0)
    }
}

impl<A: Into<AttributeSet>> std::ops::BitAndAssign<A> for AttributeSet {
    fn bitand_assign(&mut self, rhs: A) {
        self.0 &= rhs.into().0;
    }
}

impl<A: Into<AttributeSet>> std::ops::BitOr<A> for AttributeSet {
    type Output = AttributeSet;

    fn bitor(self, rhs: A) -> AttributeSet {
        AttributeSet(self.0 | rhs.into().0)
    }
}

impl<A: Into<AttributeSet>> std::ops::BitOrAssign<A> for AttributeSet {
    fn bitor_assign(&mut self, rhs: A) {
        self.0 |= rhs.into().0;
    }
}

impl<A: Into<AttributeSet>> std::ops::BitXor<A> for AttributeSet {
    type Output = AttributeSet;

    fn bitxor(self, rhs: A) -> AttributeSet {
        AttributeSet(self.0 ^ rhs.into().0)
    }
}

impl<A: Into<AttributeSet>> std::ops::BitXorAssign<A> for AttributeSet {
    fn bitxor_assign(&mut self, rhs: A) {
        self.0 ^= rhs.into().0;
    }
}

impl<A: Into<AttributeSet>> std::ops::Sub<A> for AttributeSet {
    type Output = AttributeSet;

    fn sub(self, rhs: A) -> AttributeSet {
        AttributeSet(self.0 & !rhs.into().0)
    }
}

impl<A: Into<AttributeSet>> std::ops::SubAssign<A> for AttributeSet {
    fn sub_assign(&mut self, rhs: A) {
        self.0 &= !rhs.into().0;
    }
}

impl std::ops::Not for AttributeSet {
    type Output = AttributeSet;

    fn not(self) -> AttributeSet {
        AttributeSet(!self.0 & ((1 << Attribute::COUNT) - 1))
    }
}

/// An iterator over the [`Attribute`]s in an [`AttributeSet`]
#[derive(Clone, Debug)]
pub struct AttributeSetIter {
    inner: AttributeIter,
    set: AttributeSet,
}

impl AttributeSetIter {
    fn new(set: AttributeSet) -> AttributeSetIter {
        AttributeSetIter {
            inner: Attribute::iter(),
            set,
        }
    }
}

impl Iterator for AttributeSetIter {
    type Item = Attribute;

    fn next(&mut self) -> Option<Attribute> {
        self.inner.by_ref().find(|&attr| self.set.contains(attr))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.inner.size_hint().1)
    }
}

impl DoubleEndedIterator for AttributeSetIter {
    fn next_back(&mut self) -> Option<Attribute> {
        self.inner.by_ref().rfind(|&attr| self.set.contains(attr))
    }
}

impl std::iter::FusedIterator for AttributeSetIter {}

/// Error returned when parsing an attribute fails
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("invalid attribute name: {0:?}")]
pub struct ParseAttributeError(
    /// The invalid attribute string
    pub String,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_ended_iteration() {
        let attrs = Attribute::Bold | Attribute::Frame | Attribute::Reverse | Attribute::Strike;
        let mut iter = attrs.into_iter();
        assert_eq!(iter.next(), Some(Attribute::Bold));
        assert_eq!(iter.next_back(), Some(Attribute::Frame));
        assert_eq!(iter.next(), Some(Attribute::Reverse));
        assert_eq!(iter.next_back(), Some(Attribute::Strike));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
