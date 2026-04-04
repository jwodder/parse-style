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
/// strings from the above list.  For values with two strings, the longer
/// one is used, unless the alternate display is selected with `"{:#}"`, in
/// which case the shorter (if any) is used.
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

    /// Return the short name of the attribute, or the long name if it doesn't
    /// have a short name
    ///
    /// # Example
    ///
    /// ```
    /// use parse_style::Attribute;
    ///
    /// assert_eq!(Attribute::Bold.as_short_str(), "b");
    /// assert_eq!(Attribute::Blink.as_short_str(), "blink");
    /// ```
    pub fn as_short_str(self) -> &'static str {
        match self {
            Attribute::Bold => "b",
            Attribute::Dim => "d",
            Attribute::Italic => "i",
            Attribute::Underline => "u",
            Attribute::Blink => "blink",
            Attribute::Blink2 => "blink2",
            Attribute::Reverse => "r",
            Attribute::Conceal => "c",
            Attribute::Strike => "s",
            Attribute::Underline2 => "uu",
            Attribute::Frame => "frame",
            Attribute::Encircle => "encircle",
            Attribute::Overline => "overline",
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str(self.as_short_str())
        } else {
            f.write_str(self.as_str())
        }
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

    /// Returns the number of [`Attribute`]s in the set
    pub fn len(self) -> usize {
        let qty = self.0.count_ones();
        match usize::try_from(qty) {
            Ok(sz) => sz,
            Err(_) => unreachable!("The number of bits in a u16 should fit in a usize"),
        }
    }

    /// Test whether the set contains all [`Attribute`]s
    pub fn is_all(self) -> bool {
        self == Self::ALL
    }

    /// Test whether the set contains the given [`Attribute`]
    pub fn contains(self, attr: Attribute) -> bool {
        self.0 & (attr as u16) != 0
    }

    /// Adds the given [`Attribute`] to the set if not already present.
    ///
    /// Returns `true` if the given `Attribute` was not already in the set.
    ///
    /// # Example
    ///
    /// ```
    /// use parse_style::{Attribute, AttributeSet};
    ///
    /// let mut attrset = AttributeSet::new();
    /// assert!(!attrset.contains(Attribute::Bold));
    /// assert!(attrset.insert(Attribute::Bold));
    /// assert!(attrset.contains(Attribute::Bold));
    /// assert!(!attrset.insert(Attribute::Bold));
    /// assert!(attrset.contains(Attribute::Bold));
    /// ```
    pub fn insert(&mut self, attr: Attribute) -> bool {
        let attr = attr as u16;
        let adding = (self.0 & attr) == 0;
        self.0 |= attr;
        adding
    }

    /// Removes the given [`Attribute`] from the set if present.
    ///
    /// Returns `true` if the given `Attribute` was present in the set.
    ///
    /// # Example
    ///
    /// ```
    /// use parse_style::{Attribute, AttributeSet};
    ///
    /// let mut attrset = AttributeSet::from([Attribute::Bold]);
    /// assert!(attrset.contains(Attribute::Bold));
    /// assert!(attrset.remove(Attribute::Bold));
    /// assert!(!attrset.contains(Attribute::Bold));
    /// assert!(!attrset.remove(Attribute::Bold));
    /// assert!(!attrset.contains(Attribute::Bold));
    /// ```
    pub fn remove(&mut self, attr: Attribute) -> bool {
        let attr = attr as u16;
        let present = (self.0 & attr) != 0;
        self.0 &= !attr;
        present
    }

    /// Removes all [`Attribute`]s from the set
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Returns true if `self` and `other` are disjoint, i.e., if there is no
    /// [`Attribute`] that is in both sets.
    ///
    /// # Examples
    ///
    /// ```
    /// use parse_style::{Attribute, AttributeSet};
    ///
    /// let attrset1 = AttributeSet::from([Attribute::Bold,
    /// Attribute::Italic]);
    /// let attrset2 = AttributeSet::from([Attribute::Underline,
    /// Attribute::Blink]);
    /// assert!(attrset1.is_disjoint(attrset2));
    /// assert!(attrset2.is_disjoint(attrset1));
    /// ```
    ///
    /// ```
    /// use parse_style::{Attribute, AttributeSet};
    ///
    /// let attrset1 = AttributeSet::from([Attribute::Bold,
    /// Attribute::Italic]);
    /// let attrset2 = AttributeSet::from([Attribute::Bold,
    /// Attribute::Underline]);
    /// assert!(!attrset1.is_disjoint(attrset2));
    /// assert!(!attrset2.is_disjoint(attrset1));
    /// ```
    pub fn is_disjoint(self, other: AttributeSet) -> bool {
        self.0 & other.0 == 0
    }

    /// Returns `true` if `self` is a subset of `other`
    ///
    /// # Examples
    ///
    /// ```
    /// use parse_style::{Attribute, AttributeSet};
    ///
    /// let attrset1 = AttributeSet::from([Attribute::Bold]);
    /// let attrset2 = AttributeSet::from([Attribute::Bold,
    /// Attribute::Underline]);
    /// assert!(attrset1.is_subset(attrset2));
    /// assert!(!attrset2.is_subset(attrset1));
    /// ```
    ///
    /// ```
    /// use parse_style::{Attribute, AttributeSet};
    ///
    /// let attrset1 = AttributeSet::from([Attribute::Bold,
    /// Attribute::Italic]);
    /// let attrset2 = AttributeSet::from([Attribute::Bold,
    /// Attribute::Underline]);
    /// assert!(!attrset1.is_subset(attrset2));
    /// assert!(!attrset2.is_subset(attrset1));
    /// ```
    pub fn is_subset(self, other: AttributeSet) -> bool {
        self.0 & other.0 == self.0
    }

    /// Returns `true` if `self` is a superset of `other`
    ///
    /// # Examples
    ///
    /// ```
    /// use parse_style::{Attribute, AttributeSet};
    ///
    /// let attrset1 = AttributeSet::from([Attribute::Bold]);
    /// let attrset2 = AttributeSet::from([Attribute::Bold,
    /// Attribute::Underline]);
    /// assert!(!attrset1.is_superset(attrset2));
    /// assert!(attrset2.is_superset(attrset1));
    /// ```
    ///
    /// ```
    /// use parse_style::{Attribute, AttributeSet};
    ///
    /// let attrset1 = AttributeSet::from([Attribute::Bold,
    /// Attribute::Italic]);
    /// let attrset2 = AttributeSet::from([Attribute::Bold,
    /// Attribute::Underline]);
    /// assert!(!attrset1.is_superset(attrset2));
    /// assert!(!attrset2.is_superset(attrset1));
    /// ```
    pub fn is_superset(self, other: AttributeSet) -> bool {
        self.0 & other.0 == other.0
    }
}

impl From<Attribute> for AttributeSet {
    fn from(value: Attribute) -> AttributeSet {
        AttributeSet(value as u16)
    }
}

impl<const N: usize> From<[Attribute; N]> for AttributeSet {
    fn from(value: [Attribute; N]) -> AttributeSet {
        AttributeSet::from_iter(value)
    }
}

impl IntoIterator for AttributeSet {
    type Item = Attribute;
    type IntoIter = AttributeSetIter;

    fn into_iter(self) -> AttributeSetIter {
        AttributeSetIter::new(self)
    }
}

impl FromIterator<Attribute> for AttributeSet {
    fn from_iter<I: IntoIterator<Item = Attribute>>(iter: I) -> Self {
        iter.into_iter()
            .fold(AttributeSet::new(), |set, attr| set | attr)
    }
}

impl Extend<Attribute> for AttributeSet {
    fn extend<I: IntoIterator<Item = Attribute>>(&mut self, iter: I) {
        for attr in iter {
            *self |= attr;
        }
    }
}

#[cfg(feature = "anstyle")]
#[cfg_attr(docsrs, doc(cfg(feature = "anstyle")))]
impl From<AttributeSet> for anstyle::Effects {
    /// Convert an `AttributeSet` to an [`anstyle::Effects`]
    ///
    /// # Data Loss
    ///
    /// The following attributes are discarded during conversion, as they have
    /// no `anstyle::Effects` equivalents:
    ///
    /// - [`Attribute::Blink2`]
    /// - [`Attribute::Frame`]
    /// - [`Attribute::Encircle`]
    /// - [`Attribute::Overline`]
    fn from(value: AttributeSet) -> anstyle::Effects {
        let mut efs = anstyle::Effects::new();
        for attr in value {
            match attr {
                Attribute::Bold => efs |= anstyle::Effects::BOLD,
                Attribute::Dim => efs |= anstyle::Effects::DIMMED,
                Attribute::Italic => efs |= anstyle::Effects::ITALIC,
                Attribute::Underline => efs |= anstyle::Effects::UNDERLINE,
                Attribute::Blink => efs |= anstyle::Effects::BLINK,
                Attribute::Blink2 => (),
                Attribute::Reverse => efs |= anstyle::Effects::INVERT,
                Attribute::Conceal => efs |= anstyle::Effects::HIDDEN,
                Attribute::Strike => efs |= anstyle::Effects::STRIKETHROUGH,
                Attribute::Underline2 => efs |= anstyle::Effects::DOUBLE_UNDERLINE,
                Attribute::Frame => (),
                Attribute::Encircle => (),
                Attribute::Overline => (),
            }
        }
        efs
    }
}

#[cfg(feature = "anstyle")]
#[cfg_attr(docsrs, doc(cfg(feature = "anstyle")))]
impl From<anstyle::Effects> for AttributeSet {
    /// Convert an [`anstyle::Effects`] to an `AttributeSet`
    ///
    ///
    /// # Data Loss
    ///
    /// The following effects are discarded during conversion, as they have no
    /// `Attribute` equivalents:
    ///
    /// - [`anstyle::Effects::CURLY_UNDERLINE`]
    /// - [`anstyle::Effects::DOTTED_UNDERLINE`]
    /// - [`anstyle::Effects::DASHED_UNDERLINE`]
    fn from(value: anstyle::Effects) -> AttributeSet {
        let mut set = AttributeSet::new();
        for eff in value.iter() {
            match eff {
                anstyle::Effects::BOLD => set |= Attribute::Bold,
                anstyle::Effects::DIMMED => set |= Attribute::Dim,
                anstyle::Effects::ITALIC => set |= Attribute::Italic,
                anstyle::Effects::UNDERLINE => set |= Attribute::Underline,
                anstyle::Effects::DOUBLE_UNDERLINE => set |= Attribute::Underline2,
                anstyle::Effects::CURLY_UNDERLINE => (),
                anstyle::Effects::DOTTED_UNDERLINE => (),
                anstyle::Effects::DASHED_UNDERLINE => (),
                anstyle::Effects::BLINK => set |= Attribute::Blink,
                anstyle::Effects::INVERT => set |= Attribute::Reverse,
                anstyle::Effects::HIDDEN => set |= Attribute::Conceal,
                anstyle::Effects::STRIKETHROUGH => set |= Attribute::Strike,
                // Because an `Effects` can be either a single effect or
                // multiple, we need a catch-all arm here, even though the
                // iterator will only yield single effects.
                _ => (),
            }
        }
        set
    }
}

#[cfg(feature = "crossterm")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossterm")))]
impl From<AttributeSet> for crossterm::style::Attributes {
    /// Convert an `AttributeSet` to a [`crossterm::style::Attributes`] that
    /// enables the input attributes
    fn from(value: AttributeSet) -> crossterm::style::Attributes {
        use crossterm::style::Attribute as CrossAttrib;
        let mut attributes = crossterm::style::Attributes::none();
        for attr in value {
            let ca = match attr {
                Attribute::Bold => CrossAttrib::Bold,
                Attribute::Dim => CrossAttrib::Dim,
                Attribute::Italic => CrossAttrib::Italic,
                Attribute::Underline => CrossAttrib::Underlined,
                Attribute::Blink => CrossAttrib::SlowBlink,
                Attribute::Blink2 => CrossAttrib::RapidBlink,
                Attribute::Reverse => CrossAttrib::Reverse,
                Attribute::Conceal => CrossAttrib::Hidden,
                Attribute::Strike => CrossAttrib::CrossedOut,
                Attribute::Underline2 => CrossAttrib::DoubleUnderlined,
                Attribute::Frame => CrossAttrib::Framed,
                Attribute::Encircle => CrossAttrib::Encircled,
                Attribute::Overline => CrossAttrib::OverLined,
            };
            attributes.set(ca);
        }
        attributes
    }
}

#[cfg(feature = "ratatui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ratatui")))]
impl From<AttributeSet> for ratatui_core::style::Modifier {
    /// Convert an `AttributeSet` to a [`ratatui_core::style::Modifier`]
    ///
    /// # Data Loss
    ///
    /// The following attributes are discarded during conversion, as they have
    /// no `ratatui_core::style::Modifier` equivalents:
    ///
    /// - [`Attribute::Underline2`]
    /// - [`Attribute::Frame`]
    /// - [`Attribute::Encircle`]
    /// - [`Attribute::Overline`]
    fn from(value: AttributeSet) -> ratatui_core::style::Modifier {
        let mut mods = ratatui_core::style::Modifier::empty();
        for attr in value {
            match attr {
                Attribute::Bold => mods |= ratatui_core::style::Modifier::BOLD,
                Attribute::Dim => mods |= ratatui_core::style::Modifier::DIM,
                Attribute::Italic => mods |= ratatui_core::style::Modifier::ITALIC,
                Attribute::Underline => mods |= ratatui_core::style::Modifier::UNDERLINED,
                Attribute::Blink => mods |= ratatui_core::style::Modifier::SLOW_BLINK,
                Attribute::Blink2 => mods |= ratatui_core::style::Modifier::RAPID_BLINK,
                Attribute::Reverse => mods |= ratatui_core::style::Modifier::REVERSED,
                Attribute::Conceal => mods |= ratatui_core::style::Modifier::HIDDEN,
                Attribute::Strike => mods |= ratatui_core::style::Modifier::CROSSED_OUT,
                Attribute::Underline2 => (),
                Attribute::Frame => (),
                Attribute::Encircle => (),
                Attribute::Overline => (),
            }
        }
        mods
    }
}

#[cfg(feature = "ratatui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ratatui")))]
impl From<ratatui_core::style::Modifier> for AttributeSet {
    /// Convert a [`ratatui_core::style::Modifier`] to an `AttributeSet`
    fn from(value: ratatui_core::style::Modifier) -> AttributeSet {
        let mut set = AttributeSet::new();
        for m in value.iter() {
            match m {
                ratatui_core::style::Modifier::BOLD => set |= Attribute::Bold,
                ratatui_core::style::Modifier::DIM => set |= Attribute::Dim,
                ratatui_core::style::Modifier::ITALIC => set |= Attribute::Italic,
                ratatui_core::style::Modifier::UNDERLINED => set |= Attribute::Underline,
                ratatui_core::style::Modifier::SLOW_BLINK => set |= Attribute::Blink,
                ratatui_core::style::Modifier::RAPID_BLINK => set |= Attribute::Blink,
                ratatui_core::style::Modifier::REVERSED => set |= Attribute::Reverse,
                ratatui_core::style::Modifier::HIDDEN => set |= Attribute::Conceal,
                ratatui_core::style::Modifier::CROSSED_OUT => set |= Attribute::Strike,
                // Because a `Modifier` can be either a single effect or
                // multiple, we need a catch-all arm here, even though the
                // iterator will only yield single modifiers.
                _ => (),
            }
        }
        set
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

    mod attribute {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case(Attribute::Bold, "bold")]
        #[case(Attribute::Dim, "dim")]
        #[case(Attribute::Italic, "italic")]
        #[case(Attribute::Underline, "underline")]
        #[case(Attribute::Blink, "blink")]
        #[case(Attribute::Blink2, "blink2")]
        #[case(Attribute::Reverse, "reverse")]
        #[case(Attribute::Conceal, "conceal")]
        #[case(Attribute::Strike, "strike")]
        #[case(Attribute::Underline2, "underline2")]
        #[case(Attribute::Frame, "frame")]
        #[case(Attribute::Encircle, "encircle")]
        #[case(Attribute::Overline, "overline")]
        fn display(#[case] attr: Attribute, #[case] s: &str) {
            assert_eq!(attr.to_string(), s);
        }

        #[rstest]
        #[case(Attribute::Bold, "b")]
        #[case(Attribute::Dim, "d")]
        #[case(Attribute::Italic, "i")]
        #[case(Attribute::Underline, "u")]
        #[case(Attribute::Blink, "blink")]
        #[case(Attribute::Blink2, "blink2")]
        #[case(Attribute::Reverse, "r")]
        #[case(Attribute::Conceal, "c")]
        #[case(Attribute::Strike, "s")]
        #[case(Attribute::Underline2, "uu")]
        #[case(Attribute::Frame, "frame")]
        #[case(Attribute::Encircle, "encircle")]
        #[case(Attribute::Overline, "overline")]
        fn alt_display(#[case] attr: Attribute, #[case] s: &str) {
            assert_eq!(format!("{attr:#}"), s);
        }
    }

    mod attribute_set {
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
}
