use super::ParseColorError;
use crate::color::Color;
use crate::style::Style;
use crate::util::strip_nocase_prefix;
use phf::{Map, phf_map};
use std::fmt;
use unicase::UniCase;

/// Colors in a 256-value (8-bit) palette
///
/// Constants are provided for the initial 16 colors.
///
/// `Color256` values can be [parsed][std::str::FromStr] from case-insensitive
/// names (see [`Color256::name()`]) or from strings of the form
/// `"color({index})"`.
///
/// `Color256` values are [displayed][std::fmt::Display] as their lowercase
/// names or, for colors without names, as strings of the form
/// `"color({index})"`.
///
/// # Examples
///
/// ```
/// use parse_style::Color256;
///
/// assert_eq!("color(2)".parse::<Color256>().unwrap(), Color256(2));
/// assert_eq!("green".parse::<Color256>().unwrap(), Color256(2));
/// assert_eq!("GREEN".parse::<Color256>().unwrap(), Color256(2));
///
/// assert_eq!(Color256(2).to_string(), "green");
/// assert_eq!(Color256(42).to_string(), "color(42)");
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Color256(pub u8);

impl Color256 {
    /// Color 0
    pub const BLACK: Color256 = Color256(0);

    /// Color 1, also known as "dark red"
    pub const RED: Color256 = Color256(1);

    /// Color 2, also known as "dark green"
    pub const GREEN: Color256 = Color256(2);

    /// Color 3, also known as "dark yellow"
    pub const YELLOW: Color256 = Color256(3);

    /// Color 4, also known as "dark blue"
    pub const BLUE: Color256 = Color256(4);

    /// Color 5, also known as "dark magenta"
    pub const MAGENTA: Color256 = Color256(5);

    /// Color 6, also known as "dark cyan"
    pub const CYAN: Color256 = Color256(6);

    /// Color 7, also known as "grey"
    pub const WHITE: Color256 = Color256(7);

    /// Color 8, also known as "dark grey" or "grey"
    pub const BRIGHT_BLACK: Color256 = Color256(8);

    /// Color 9
    pub const BRIGHT_RED: Color256 = Color256(9);

    /// Color 10
    pub const BRIGHT_GREEN: Color256 = Color256(10);

    /// Color 11
    pub const BRIGHT_YELLOW: Color256 = Color256(11);

    /// Color 12
    pub const BRIGHT_BLUE: Color256 = Color256(12);

    /// Color 13
    pub const BRIGHT_MAGENTA: Color256 = Color256(13);

    /// Color 14
    pub const BRIGHT_CYAN: Color256 = Color256(14);

    /// Color 15, also known as "white"
    pub const BRIGHT_WHITE: Color256 = Color256(15);

    /// Return the lowercase name of the color as recognized by `rich`.  See
    /// [this page][table] for the list of color names.
    ///
    /// Note that not all colors have a name.  Also note that some greyscale
    /// colors have two names, one using the spelling "gray" and the other
    /// "grey"; this method uses the "gray" spellings.
    ///
    /// [table]: https://rich.readthedocs.io/en/stable/appendix/colors.html
    ///
    /// # Examples
    ///
    /// ```
    /// use parse_style::Color256;
    ///
    /// assert_eq!(Color256::BRIGHT_YELLOW.name(), Some("bright_yellow"));
    /// assert_eq!(Color256(42).name(), None);
    /// assert_eq!(Color256(59).name(), Some("gray37"));
    /// assert_eq!(Color256(118).name(), Some("chartreuse1"));
    /// ```
    pub fn name(self) -> Option<&'static str> {
        match self.0 {
            0 => Some("black"),
            1 => Some("red"),
            2 => Some("green"),
            3 => Some("yellow"),
            4 => Some("blue"),
            5 => Some("magenta"),
            6 => Some("cyan"),
            7 => Some("white"),
            8 => Some("bright_black"),
            9 => Some("bright_red"),
            10 => Some("bright_green"),
            11 => Some("bright_yellow"),
            12 => Some("bright_blue"),
            13 => Some("bright_magenta"),
            14 => Some("bright_cyan"),
            15 => Some("bright_white"),
            16 => Some("gray0"),
            17 => Some("navy_blue"),
            18 => Some("dark_blue"),
            19 => None,
            20 => Some("blue3"),
            21 => Some("blue1"),
            22 => Some("dark_green"),
            23 => None,
            24 => None,
            25 => Some("deep_sky_blue4"),
            26 => Some("dodger_blue3"),
            27 => Some("dodger_blue2"),
            28 => Some("green4"),
            29 => Some("spring_green4"),
            30 => Some("turquoise4"),
            31 => None,
            32 => Some("deep_sky_blue3"),
            33 => Some("dodger_blue1"),
            34 => None,
            35 => None,
            36 => Some("dark_cyan"),
            37 => Some("light_sea_green"),
            38 => Some("deep_sky_blue2"),
            39 => Some("deep_sky_blue1"),
            40 => Some("green3"),
            41 => Some("spring_green3"),
            42 => None,
            43 => Some("cyan3"),
            44 => Some("dark_turquoise"),
            45 => Some("turquoise2"),
            46 => Some("green1"),
            47 => Some("spring_green2"),
            48 => Some("spring_green1"),
            49 => Some("medium_spring_green"),
            50 => Some("cyan2"),
            51 => Some("cyan1"),
            52 => None,
            53 => None,
            54 => None,
            55 => Some("purple4"),
            56 => Some("purple3"),
            57 => Some("blue_violet"),
            58 => None,
            59 => Some("gray37"),
            60 => Some("medium_purple4"),
            61 => None,
            62 => Some("slate_blue3"),
            63 => Some("royal_blue1"),
            64 => Some("chartreuse4"),
            65 => None,
            66 => Some("pale_turquoise4"),
            67 => Some("steel_blue"),
            68 => Some("steel_blue3"),
            69 => Some("cornflower_blue"),
            70 => None,
            71 => Some("dark_sea_green4"),
            72 => None,
            73 => Some("cadet_blue"),
            74 => Some("sky_blue3"),
            75 => None,
            76 => Some("chartreuse3"),
            77 => None,
            78 => Some("sea_green3"),
            79 => Some("aquamarine3"),
            80 => Some("medium_turquoise"),
            81 => Some("steel_blue1"),
            82 => None,
            83 => Some("sea_green2"),
            84 => None,
            85 => Some("sea_green1"),
            86 => None,
            87 => Some("dark_slate_gray2"),
            88 => Some("dark_red"),
            89 => None,
            90 => None,
            91 => Some("dark_magenta"),
            92 => None,
            93 => None,
            94 => Some("orange4"),
            95 => Some("light_pink4"),
            96 => Some("plum4"),
            97 => None,
            98 => Some("medium_purple3"),
            99 => Some("slate_blue1"),
            100 => None,
            101 => Some("wheat4"),
            102 => Some("gray53"),
            103 => Some("light_slate_gray"),
            104 => Some("medium_purple"),
            105 => Some("light_slate_blue"),
            106 => Some("yellow4"),
            107 => None,
            108 => Some("dark_sea_green"),
            109 => None,
            110 => Some("light_sky_blue3"),
            111 => Some("sky_blue2"),
            112 => Some("chartreuse2"),
            113 => None,
            114 => Some("pale_green3"),
            115 => None,
            116 => Some("dark_slate_gray3"),
            117 => Some("sky_blue1"),
            118 => Some("chartreuse1"),
            119 => None,
            120 => Some("light_green"),
            121 => None,
            122 => Some("aquamarine1"),
            123 => Some("dark_slate_gray1"),
            124 => None,
            125 => Some("deep_pink4"),
            126 => Some("medium_violet_red"),
            127 => None,
            128 => Some("dark_violet"),
            129 => Some("purple"),
            130 => None,
            131 => None,
            132 => None,
            133 => Some("medium_orchid3"),
            134 => Some("medium_orchid"),
            135 => None,
            136 => Some("dark_goldenrod"),
            137 => None,
            138 => Some("rosy_brown"),
            139 => Some("gray63"),
            140 => Some("medium_purple2"),
            141 => Some("medium_purple1"),
            142 => None,
            143 => Some("dark_khaki"),
            144 => Some("navajo_white3"),
            145 => Some("gray69"),
            146 => Some("light_steel_blue3"),
            147 => Some("light_steel_blue"),
            148 => None,
            149 => Some("dark_olive_green3"),
            150 => Some("dark_sea_green3"),
            151 => None,
            152 => Some("light_cyan3"),
            153 => Some("light_sky_blue1"),
            154 => Some("green_yellow"),
            155 => Some("dark_olive_green2"),
            156 => Some("pale_green1"),
            157 => Some("dark_sea_green2"),
            158 => None,
            159 => Some("pale_turquoise1"),
            160 => Some("red3"),
            161 => None,
            162 => Some("deep_pink3"),
            163 => None,
            164 => Some("magenta3"),
            165 => None,
            166 => Some("dark_orange3"),
            167 => Some("indian_red"),
            168 => Some("hot_pink3"),
            169 => Some("hot_pink2"),
            170 => Some("orchid"),
            171 => None,
            172 => Some("orange3"),
            173 => Some("light_salmon3"),
            174 => Some("light_pink3"),
            175 => Some("pink3"),
            176 => Some("plum3"),
            177 => Some("violet"),
            178 => Some("gold3"),
            179 => Some("light_goldenrod3"),
            180 => Some("tan"),
            181 => Some("misty_rose3"),
            182 => Some("thistle3"),
            183 => Some("plum2"),
            184 => Some("yellow3"),
            185 => Some("khaki3"),
            186 => None,
            187 => Some("light_yellow3"),
            188 => Some("gray84"),
            189 => Some("light_steel_blue1"),
            190 => Some("yellow2"),
            191 => None,
            192 => Some("dark_olive_green1"),
            193 => Some("dark_sea_green1"),
            194 => Some("honeydew2"),
            195 => Some("light_cyan1"),
            196 => Some("red1"),
            197 => Some("deep_pink2"),
            198 => None,
            199 => Some("deep_pink1"),
            200 => Some("magenta2"),
            201 => Some("magenta1"),
            202 => Some("orange_red1"),
            203 => None,
            204 => Some("indian_red1"),
            205 => None,
            206 => Some("hot_pink"),
            207 => Some("medium_orchid1"),
            208 => Some("dark_orange"),
            209 => Some("salmon1"),
            210 => Some("light_coral"),
            211 => Some("pale_violet_red1"),
            212 => Some("orchid2"),
            213 => Some("orchid1"),
            214 => Some("orange1"),
            215 => Some("sandy_brown"),
            216 => Some("light_salmon1"),
            217 => Some("light_pink1"),
            218 => Some("pink1"),
            219 => Some("plum1"),
            220 => Some("gold1"),
            221 => None,
            222 => Some("light_goldenrod2"),
            223 => Some("navajo_white1"),
            224 => Some("misty_rose1"),
            225 => Some("thistle1"),
            226 => Some("yellow1"),
            227 => Some("light_goldenrod1"),
            228 => Some("khaki1"),
            229 => Some("wheat1"),
            230 => Some("cornsilk1"),
            231 => Some("gray100"),
            232 => Some("gray3"),
            233 => Some("gray7"),
            234 => Some("gray11"),
            235 => Some("gray15"),
            236 => Some("gray19"),
            237 => Some("gray23"),
            238 => Some("gray27"),
            239 => Some("gray30"),
            240 => Some("gray35"),
            241 => Some("gray39"),
            242 => Some("gray42"),
            243 => Some("gray46"),
            244 => Some("gray50"),
            245 => Some("gray54"),
            246 => Some("gray58"),
            247 => Some("gray62"),
            248 => Some("gray66"),
            249 => Some("gray70"),
            250 => Some("gray74"),
            251 => Some("gray78"),
            252 => Some("gray82"),
            253 => Some("gray85"),
            254 => Some("gray89"),
            255 => Some("gray93"),
        }
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

impl From<u8> for Color256 {
    fn from(value: u8) -> Color256 {
        Color256(value)
    }
}

impl From<Color256> for u8 {
    fn from(value: Color256) -> u8 {
        value.0
    }
}

#[cfg(feature = "anstyle")]
#[cfg_attr(docsrs, doc(cfg(feature = "anstyle")))]
impl From<anstyle::AnsiColor> for Color256 {
    /// Convert an [`anstyle::AnsiColor`] to a `Color256`
    fn from(value: anstyle::AnsiColor) -> Color256 {
        Color256(value as u8)
    }
}

#[cfg(feature = "anstyle")]
#[cfg_attr(docsrs, doc(cfg(feature = "anstyle")))]
impl From<Color256> for anstyle::Ansi256Color {
    /// Convert a `Color256` to an [`anstyle::Ansi256Color`]
    fn from(value: Color256) -> anstyle::Ansi256Color {
        anstyle::Ansi256Color(value.0)
    }
}

#[cfg(feature = "anstyle")]
#[cfg_attr(docsrs, doc(cfg(feature = "anstyle")))]
impl From<anstyle::Ansi256Color> for Color256 {
    /// Convert an [`anstyle::Ansi256Color`] to a `Color256`
    fn from(value: anstyle::Ansi256Color) -> Color256 {
        Color256(value.0)
    }
}

#[cfg(feature = "crossterm")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossterm")))]
impl From<Color256> for crossterm::style::Color {
    /// Convert a `Color256` to a [`crossterm::style::Color`]
    fn from(value: Color256) -> crossterm::style::Color {
        crossterm::style::Color::AnsiValue(value.0)
    }
}

#[cfg(feature = "ratatui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ratatui")))]
impl From<Color256> for ratatui_core::style::Color {
    /// Convert a `Color256` to a [`ratatui_core::style::Color`]
    fn from(value: Color256) -> ratatui_core::style::Color {
        ratatui_core::style::Color::Indexed(value.0)
    }
}

impl fmt::Display for Color256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => write!(f, "{name}"),
            None => write!(f, "color({})", self.0),
        }
    }
}

impl std::str::FromStr for Color256 {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Color256, ParseColorError> {
        if let Some(color) = BY_NAME.get(&UniCase::ascii(s)).copied() {
            Ok(color)
        } else if let Some(index) = strip_nocase_prefix(s, "color(")
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.parse::<u8>().ok())
        {
            Ok(Color256(index))
        } else {
            Err(ParseColorError(s.to_owned()))
        }
    }
}

static BY_NAME: Map<UniCase<&'static str>, Color256> = phf_map! {
    UniCase::ascii("black") => Color256(0),
    UniCase::ascii("red") => Color256(1),
    UniCase::ascii("green") => Color256(2),
    UniCase::ascii("yellow") => Color256(3),
    UniCase::ascii("blue") => Color256(4),
    UniCase::ascii("magenta") => Color256(5),
    UniCase::ascii("cyan") => Color256(6),
    UniCase::ascii("white") => Color256(7),
    UniCase::ascii("bright_black") => Color256(8),
    UniCase::ascii("bright_red") => Color256(9),
    UniCase::ascii("bright_green") => Color256(10),
    UniCase::ascii("bright_yellow") => Color256(11),
    UniCase::ascii("bright_blue") => Color256(12),
    UniCase::ascii("bright_magenta") => Color256(13),
    UniCase::ascii("bright_cyan") => Color256(14),
    UniCase::ascii("bright_white") => Color256(15),
    UniCase::ascii("grey0") => Color256(16),
    UniCase::ascii("gray0") => Color256(16),
    UniCase::ascii("navy_blue") => Color256(17),
    UniCase::ascii("dark_blue") => Color256(18),
    UniCase::ascii("blue3") => Color256(20),
    UniCase::ascii("blue1") => Color256(21),
    UniCase::ascii("dark_green") => Color256(22),
    UniCase::ascii("deep_sky_blue4") => Color256(25),
    UniCase::ascii("dodger_blue3") => Color256(26),
    UniCase::ascii("dodger_blue2") => Color256(27),
    UniCase::ascii("green4") => Color256(28),
    UniCase::ascii("spring_green4") => Color256(29),
    UniCase::ascii("turquoise4") => Color256(30),
    UniCase::ascii("deep_sky_blue3") => Color256(32),
    UniCase::ascii("dodger_blue1") => Color256(33),
    UniCase::ascii("green3") => Color256(40),
    UniCase::ascii("spring_green3") => Color256(41),
    UniCase::ascii("dark_cyan") => Color256(36),
    UniCase::ascii("light_sea_green") => Color256(37),
    UniCase::ascii("deep_sky_blue2") => Color256(38),
    UniCase::ascii("deep_sky_blue1") => Color256(39),
    UniCase::ascii("spring_green2") => Color256(47),
    UniCase::ascii("cyan3") => Color256(43),
    UniCase::ascii("dark_turquoise") => Color256(44),
    UniCase::ascii("turquoise2") => Color256(45),
    UniCase::ascii("green1") => Color256(46),
    UniCase::ascii("spring_green1") => Color256(48),
    UniCase::ascii("medium_spring_green") => Color256(49),
    UniCase::ascii("cyan2") => Color256(50),
    UniCase::ascii("cyan1") => Color256(51),
    UniCase::ascii("dark_red") => Color256(88),
    UniCase::ascii("deep_pink4") => Color256(125),
    UniCase::ascii("purple4") => Color256(55),
    UniCase::ascii("purple3") => Color256(56),
    UniCase::ascii("blue_violet") => Color256(57),
    UniCase::ascii("orange4") => Color256(94),
    UniCase::ascii("grey37") => Color256(59),
    UniCase::ascii("gray37") => Color256(59),
    UniCase::ascii("medium_purple4") => Color256(60),
    UniCase::ascii("slate_blue3") => Color256(62),
    UniCase::ascii("royal_blue1") => Color256(63),
    UniCase::ascii("chartreuse4") => Color256(64),
    UniCase::ascii("dark_sea_green4") => Color256(71),
    UniCase::ascii("pale_turquoise4") => Color256(66),
    UniCase::ascii("steel_blue") => Color256(67),
    UniCase::ascii("steel_blue3") => Color256(68),
    UniCase::ascii("cornflower_blue") => Color256(69),
    UniCase::ascii("chartreuse3") => Color256(76),
    UniCase::ascii("cadet_blue") => Color256(73),
    UniCase::ascii("sky_blue3") => Color256(74),
    UniCase::ascii("steel_blue1") => Color256(81),
    UniCase::ascii("pale_green3") => Color256(114),
    UniCase::ascii("sea_green3") => Color256(78),
    UniCase::ascii("aquamarine3") => Color256(79),
    UniCase::ascii("medium_turquoise") => Color256(80),
    UniCase::ascii("chartreuse2") => Color256(112),
    UniCase::ascii("sea_green2") => Color256(83),
    UniCase::ascii("sea_green1") => Color256(85),
    UniCase::ascii("aquamarine1") => Color256(122),
    UniCase::ascii("dark_slate_gray2") => Color256(87),
    UniCase::ascii("dark_magenta") => Color256(91),
    UniCase::ascii("dark_violet") => Color256(128),
    UniCase::ascii("purple") => Color256(129),
    UniCase::ascii("light_pink4") => Color256(95),
    UniCase::ascii("plum4") => Color256(96),
    UniCase::ascii("medium_purple3") => Color256(98),
    UniCase::ascii("slate_blue1") => Color256(99),
    UniCase::ascii("yellow4") => Color256(106),
    UniCase::ascii("wheat4") => Color256(101),
    UniCase::ascii("grey53") => Color256(102),
    UniCase::ascii("gray53") => Color256(102),
    UniCase::ascii("light_slate_grey") => Color256(103),
    UniCase::ascii("light_slate_gray") => Color256(103),
    UniCase::ascii("medium_purple") => Color256(104),
    UniCase::ascii("light_slate_blue") => Color256(105),
    UniCase::ascii("dark_olive_green3") => Color256(149),
    UniCase::ascii("dark_sea_green") => Color256(108),
    UniCase::ascii("light_sky_blue3") => Color256(110),
    UniCase::ascii("sky_blue2") => Color256(111),
    UniCase::ascii("dark_sea_green3") => Color256(150),
    UniCase::ascii("dark_slate_gray3") => Color256(116),
    UniCase::ascii("sky_blue1") => Color256(117),
    UniCase::ascii("chartreuse1") => Color256(118),
    UniCase::ascii("light_green") => Color256(120),
    UniCase::ascii("pale_green1") => Color256(156),
    UniCase::ascii("dark_slate_gray1") => Color256(123),
    UniCase::ascii("red3") => Color256(160),
    UniCase::ascii("medium_violet_red") => Color256(126),
    UniCase::ascii("magenta3") => Color256(164),
    UniCase::ascii("dark_orange3") => Color256(166),
    UniCase::ascii("indian_red") => Color256(167),
    UniCase::ascii("hot_pink3") => Color256(168),
    UniCase::ascii("medium_orchid3") => Color256(133),
    UniCase::ascii("medium_orchid") => Color256(134),
    UniCase::ascii("medium_purple2") => Color256(140),
    UniCase::ascii("dark_goldenrod") => Color256(136),
    UniCase::ascii("light_salmon3") => Color256(173),
    UniCase::ascii("rosy_brown") => Color256(138),
    UniCase::ascii("grey63") => Color256(139),
    UniCase::ascii("gray63") => Color256(139),
    UniCase::ascii("medium_purple1") => Color256(141),
    UniCase::ascii("gold3") => Color256(178),
    UniCase::ascii("dark_khaki") => Color256(143),
    UniCase::ascii("navajo_white3") => Color256(144),
    UniCase::ascii("grey69") => Color256(145),
    UniCase::ascii("gray69") => Color256(145),
    UniCase::ascii("light_steel_blue3") => Color256(146),
    UniCase::ascii("light_steel_blue") => Color256(147),
    UniCase::ascii("yellow3") => Color256(184),
    UniCase::ascii("dark_sea_green2") => Color256(157),
    UniCase::ascii("light_cyan3") => Color256(152),
    UniCase::ascii("light_sky_blue1") => Color256(153),
    UniCase::ascii("green_yellow") => Color256(154),
    UniCase::ascii("dark_olive_green2") => Color256(155),
    UniCase::ascii("dark_sea_green1") => Color256(193),
    UniCase::ascii("pale_turquoise1") => Color256(159),
    UniCase::ascii("deep_pink3") => Color256(162),
    UniCase::ascii("magenta2") => Color256(200),
    UniCase::ascii("hot_pink2") => Color256(169),
    UniCase::ascii("orchid") => Color256(170),
    UniCase::ascii("medium_orchid1") => Color256(207),
    UniCase::ascii("orange3") => Color256(172),
    UniCase::ascii("light_pink3") => Color256(174),
    UniCase::ascii("pink3") => Color256(175),
    UniCase::ascii("plum3") => Color256(176),
    UniCase::ascii("violet") => Color256(177),
    UniCase::ascii("light_goldenrod3") => Color256(179),
    UniCase::ascii("tan") => Color256(180),
    UniCase::ascii("misty_rose3") => Color256(181),
    UniCase::ascii("thistle3") => Color256(182),
    UniCase::ascii("plum2") => Color256(183),
    UniCase::ascii("khaki3") => Color256(185),
    UniCase::ascii("light_goldenrod2") => Color256(222),
    UniCase::ascii("light_yellow3") => Color256(187),
    UniCase::ascii("grey84") => Color256(188),
    UniCase::ascii("gray84") => Color256(188),
    UniCase::ascii("light_steel_blue1") => Color256(189),
    UniCase::ascii("yellow2") => Color256(190),
    UniCase::ascii("dark_olive_green1") => Color256(192),
    UniCase::ascii("honeydew2") => Color256(194),
    UniCase::ascii("light_cyan1") => Color256(195),
    UniCase::ascii("red1") => Color256(196),
    UniCase::ascii("deep_pink2") => Color256(197),
    UniCase::ascii("deep_pink1") => Color256(199),
    UniCase::ascii("magenta1") => Color256(201),
    UniCase::ascii("orange_red1") => Color256(202),
    UniCase::ascii("indian_red1") => Color256(204),
    UniCase::ascii("hot_pink") => Color256(206),
    UniCase::ascii("dark_orange") => Color256(208),
    UniCase::ascii("salmon1") => Color256(209),
    UniCase::ascii("light_coral") => Color256(210),
    UniCase::ascii("pale_violet_red1") => Color256(211),
    UniCase::ascii("orchid2") => Color256(212),
    UniCase::ascii("orchid1") => Color256(213),
    UniCase::ascii("orange1") => Color256(214),
    UniCase::ascii("sandy_brown") => Color256(215),
    UniCase::ascii("light_salmon1") => Color256(216),
    UniCase::ascii("light_pink1") => Color256(217),
    UniCase::ascii("pink1") => Color256(218),
    UniCase::ascii("plum1") => Color256(219),
    UniCase::ascii("gold1") => Color256(220),
    UniCase::ascii("navajo_white1") => Color256(223),
    UniCase::ascii("misty_rose1") => Color256(224),
    UniCase::ascii("thistle1") => Color256(225),
    UniCase::ascii("yellow1") => Color256(226),
    UniCase::ascii("light_goldenrod1") => Color256(227),
    UniCase::ascii("khaki1") => Color256(228),
    UniCase::ascii("wheat1") => Color256(229),
    UniCase::ascii("cornsilk1") => Color256(230),
    UniCase::ascii("grey100") => Color256(231),
    UniCase::ascii("gray100") => Color256(231),
    UniCase::ascii("grey3") => Color256(232),
    UniCase::ascii("gray3") => Color256(232),
    UniCase::ascii("grey7") => Color256(233),
    UniCase::ascii("gray7") => Color256(233),
    UniCase::ascii("grey11") => Color256(234),
    UniCase::ascii("gray11") => Color256(234),
    UniCase::ascii("grey15") => Color256(235),
    UniCase::ascii("gray15") => Color256(235),
    UniCase::ascii("grey19") => Color256(236),
    UniCase::ascii("gray19") => Color256(236),
    UniCase::ascii("grey23") => Color256(237),
    UniCase::ascii("gray23") => Color256(237),
    UniCase::ascii("grey27") => Color256(238),
    UniCase::ascii("gray27") => Color256(238),
    UniCase::ascii("grey30") => Color256(239),
    UniCase::ascii("gray30") => Color256(239),
    UniCase::ascii("grey35") => Color256(240),
    UniCase::ascii("gray35") => Color256(240),
    UniCase::ascii("grey39") => Color256(241),
    UniCase::ascii("gray39") => Color256(241),
    UniCase::ascii("grey42") => Color256(242),
    UniCase::ascii("gray42") => Color256(242),
    UniCase::ascii("grey46") => Color256(243),
    UniCase::ascii("gray46") => Color256(243),
    UniCase::ascii("grey50") => Color256(244),
    UniCase::ascii("gray50") => Color256(244),
    UniCase::ascii("grey54") => Color256(245),
    UniCase::ascii("gray54") => Color256(245),
    UniCase::ascii("grey58") => Color256(246),
    UniCase::ascii("gray58") => Color256(246),
    UniCase::ascii("grey62") => Color256(247),
    UniCase::ascii("gray62") => Color256(247),
    UniCase::ascii("grey66") => Color256(248),
    UniCase::ascii("gray66") => Color256(248),
    UniCase::ascii("grey70") => Color256(249),
    UniCase::ascii("gray70") => Color256(249),
    UniCase::ascii("grey74") => Color256(250),
    UniCase::ascii("gray74") => Color256(250),
    UniCase::ascii("grey78") => Color256(251),
    UniCase::ascii("gray78") => Color256(251),
    UniCase::ascii("grey82") => Color256(252),
    UniCase::ascii("gray82") => Color256(252),
    UniCase::ascii("grey85") => Color256(253),
    UniCase::ascii("gray85") => Color256(253),
    UniCase::ascii("grey89") => Color256(254),
    UniCase::ascii("gray89") => Color256(254),
    UniCase::ascii("grey93") => Color256(255),
    UniCase::ascii("gray93") => Color256(255),
};

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl serde::Serialize for Color256 {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> serde::Deserialize<'de> for Color256 {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = Color256;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(r#"a color word or a string of the form "color(INT)""#)
            }

            fn visit_str<E>(self, input: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                input
                    .parse::<Color256>()
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::Str(input), &self))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("black", Color256(0))]
    #[case("BLACK", Color256(0))]
    #[case("BlAcK", Color256(0))]
    #[case("color(0)", Color256(0))]
    #[case("COLOR(0)", Color256(0))]
    #[case("gray42", Color256(242))]
    #[case("grey42", Color256(242))]
    #[case("color(242)", Color256(242))]
    fn test_parse(#[case] s: &str, #[case] color: Color256) {
        assert_eq!(s.parse::<Color256>().unwrap(), color);
    }
}
