//! Module for letters with the font weight bold and size 24.
//!
//! The bitmap font contains all unicode symbols from 0x00 to 0x17f with
//! the exception of control characters. It includes Basic Latin
//! Latin-1 Supplement and Latin extended A. This means the typical letters
//! and symbols from a QWERTZ/QWERTY keyboard plus symbols such as Ö, Ä,
//! and Ü.
//!
//! # Font License
//! * Downloaded from: <https://fonts.google.com/noto/specimen/Noto+Sans+Mono>
//! * License: SIL Open Font License (OFL) <https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL>

/// The height of each bitmap character for the given font weight
/// and size. This size corresponds to the size of the module name.
/// The font size will be a few percent smaller, because each
/// bitmap contains a small vertical padding.
#[allow(dead_code)]
pub const BITMAP_HEIGHT: usize = 24;

/// The width of each bitmap character for the given font weight
/// and size. This is a few percent less than [`BITMAP_HEIGHT`],
/// because the bitmap font doesn't contain horizontal padding.
pub const BITMAP_WIDTH: usize = 13;

/// Returns the bitmap of the given character of the pre rendered
/// "Noto Sans Mono" raster for font weight bold and font size 21px
#[inline]
pub const fn get_char(c: char) -> Option<&'static [&'static [u8]]> {
    match c {
        // letter: ' ' / 0x20
        #[cfg(feature = "unicode-basic-latin")]
        ' ' => Some(include!("../res_rasterized_characters/3990.txt")),
        // letter: '!' / 0x21
        #[cfg(feature = "unicode-basic-latin")]
        '!' => Some(include!("../res_rasterized_characters/3991.txt")),
        // letter: '"' / 0x22
        #[cfg(feature = "unicode-basic-latin")]
        '"' => Some(include!("../res_rasterized_characters/3992.txt")),
        // letter: '#' / 0x23
        #[cfg(feature = "unicode-basic-latin")]
        '#' => Some(include!("../res_rasterized_characters/3993.txt")),
        // letter: '$' / 0x24
        #[cfg(feature = "unicode-basic-latin")]
        '$' => Some(include!("../res_rasterized_characters/3994.txt")),
        // letter: '%' / 0x25
        #[cfg(feature = "unicode-basic-latin")]
        '%' => Some(include!("../res_rasterized_characters/3995.txt")),
        // letter: '&' / 0x26
        #[cfg(feature = "unicode-basic-latin")]
        '&' => Some(include!("../res_rasterized_characters/3996.txt")),
        // letter: ''' / 0x27
        #[cfg(feature = "unicode-basic-latin")]
        '\'' => Some(include!("../res_rasterized_characters/3997.txt")),
        // letter: '(' / 0x28
        #[cfg(feature = "unicode-basic-latin")]
        '(' => Some(include!("../res_rasterized_characters/3998.txt")),
        // letter: ')' / 0x29
        #[cfg(feature = "unicode-basic-latin")]
        ')' => Some(include!("../res_rasterized_characters/3999.txt")),
        // letter: '*' / 0x2a
        #[cfg(feature = "unicode-basic-latin")]
        '*' => Some(include!("../res_rasterized_characters/4000.txt")),
        // letter: '+' / 0x2b
        #[cfg(feature = "unicode-basic-latin")]
        '+' => Some(include!("../res_rasterized_characters/4001.txt")),
        // letter: ',' / 0x2c
        #[cfg(feature = "unicode-basic-latin")]
        ',' => Some(include!("../res_rasterized_characters/4002.txt")),
        // letter: '-' / 0x2d
        #[cfg(feature = "unicode-basic-latin")]
        '-' => Some(include!("../res_rasterized_characters/4003.txt")),
        // letter: '.' / 0x2e
        #[cfg(feature = "unicode-basic-latin")]
        '.' => Some(include!("../res_rasterized_characters/4004.txt")),
        // letter: '/' / 0x2f
        #[cfg(feature = "unicode-basic-latin")]
        '/' => Some(include!("../res_rasterized_characters/4005.txt")),
        // letter: '0' / 0x30
        #[cfg(feature = "unicode-basic-latin")]
        '0' => Some(include!("../res_rasterized_characters/4006.txt")),
        // letter: '1' / 0x31
        #[cfg(feature = "unicode-basic-latin")]
        '1' => Some(include!("../res_rasterized_characters/4007.txt")),
        // letter: '2' / 0x32
        #[cfg(feature = "unicode-basic-latin")]
        '2' => Some(include!("../res_rasterized_characters/4008.txt")),
        // letter: '3' / 0x33
        #[cfg(feature = "unicode-basic-latin")]
        '3' => Some(include!("../res_rasterized_characters/4009.txt")),
        // letter: '4' / 0x34
        #[cfg(feature = "unicode-basic-latin")]
        '4' => Some(include!("../res_rasterized_characters/4010.txt")),
        // letter: '5' / 0x35
        #[cfg(feature = "unicode-basic-latin")]
        '5' => Some(include!("../res_rasterized_characters/4011.txt")),
        // letter: '6' / 0x36
        #[cfg(feature = "unicode-basic-latin")]
        '6' => Some(include!("../res_rasterized_characters/4012.txt")),
        // letter: '7' / 0x37
        #[cfg(feature = "unicode-basic-latin")]
        '7' => Some(include!("../res_rasterized_characters/4013.txt")),
        // letter: '8' / 0x38
        #[cfg(feature = "unicode-basic-latin")]
        '8' => Some(include!("../res_rasterized_characters/4014.txt")),
        // letter: '9' / 0x39
        #[cfg(feature = "unicode-basic-latin")]
        '9' => Some(include!("../res_rasterized_characters/4015.txt")),
        // letter: ':' / 0x3a
        #[cfg(feature = "unicode-basic-latin")]
        ':' => Some(include!("../res_rasterized_characters/4016.txt")),
        // letter: ';' / 0x3b
        #[cfg(feature = "unicode-basic-latin")]
        ';' => Some(include!("../res_rasterized_characters/4017.txt")),
        // letter: '<' / 0x3c
        #[cfg(feature = "unicode-basic-latin")]
        '<' => Some(include!("../res_rasterized_characters/4018.txt")),
        // letter: '=' / 0x3d
        #[cfg(feature = "unicode-basic-latin")]
        '=' => Some(include!("../res_rasterized_characters/4019.txt")),
        // letter: '>' / 0x3e
        #[cfg(feature = "unicode-basic-latin")]
        '>' => Some(include!("../res_rasterized_characters/4020.txt")),
        // letter: '?' / 0x3f
        #[cfg(feature = "unicode-basic-latin")]
        '?' => Some(include!("../res_rasterized_characters/4021.txt")),
        // letter: '@' / 0x40
        #[cfg(feature = "unicode-basic-latin")]
        '@' => Some(include!("../res_rasterized_characters/4022.txt")),
        // letter: 'A' / 0x41
        #[cfg(feature = "unicode-basic-latin")]
        'A' => Some(include!("../res_rasterized_characters/4023.txt")),
        // letter: 'B' / 0x42
        #[cfg(feature = "unicode-basic-latin")]
        'B' => Some(include!("../res_rasterized_characters/4024.txt")),
        // letter: 'C' / 0x43
        #[cfg(feature = "unicode-basic-latin")]
        'C' => Some(include!("../res_rasterized_characters/4025.txt")),
        // letter: 'D' / 0x44
        #[cfg(feature = "unicode-basic-latin")]
        'D' => Some(include!("../res_rasterized_characters/4026.txt")),
        // letter: 'E' / 0x45
        #[cfg(feature = "unicode-basic-latin")]
        'E' => Some(include!("../res_rasterized_characters/4027.txt")),
        // letter: 'F' / 0x46
        #[cfg(feature = "unicode-basic-latin")]
        'F' => Some(include!("../res_rasterized_characters/4028.txt")),
        // letter: 'G' / 0x47
        #[cfg(feature = "unicode-basic-latin")]
        'G' => Some(include!("../res_rasterized_characters/4029.txt")),
        // letter: 'H' / 0x48
        #[cfg(feature = "unicode-basic-latin")]
        'H' => Some(include!("../res_rasterized_characters/4030.txt")),
        // letter: 'I' / 0x49
        #[cfg(feature = "unicode-basic-latin")]
        'I' => Some(include!("../res_rasterized_characters/4031.txt")),
        // letter: 'J' / 0x4a
        #[cfg(feature = "unicode-basic-latin")]
        'J' => Some(include!("../res_rasterized_characters/4032.txt")),
        // letter: 'K' / 0x4b
        #[cfg(feature = "unicode-basic-latin")]
        'K' => Some(include!("../res_rasterized_characters/4033.txt")),
        // letter: 'L' / 0x4c
        #[cfg(feature = "unicode-basic-latin")]
        'L' => Some(include!("../res_rasterized_characters/4034.txt")),
        // letter: 'M' / 0x4d
        #[cfg(feature = "unicode-basic-latin")]
        'M' => Some(include!("../res_rasterized_characters/4035.txt")),
        // letter: 'N' / 0x4e
        #[cfg(feature = "unicode-basic-latin")]
        'N' => Some(include!("../res_rasterized_characters/4036.txt")),
        // letter: 'O' / 0x4f
        #[cfg(feature = "unicode-basic-latin")]
        'O' => Some(include!("../res_rasterized_characters/4037.txt")),
        // letter: 'P' / 0x50
        #[cfg(feature = "unicode-basic-latin")]
        'P' => Some(include!("../res_rasterized_characters/4038.txt")),
        // letter: 'Q' / 0x51
        #[cfg(feature = "unicode-basic-latin")]
        'Q' => Some(include!("../res_rasterized_characters/4039.txt")),
        // letter: 'R' / 0x52
        #[cfg(feature = "unicode-basic-latin")]
        'R' => Some(include!("../res_rasterized_characters/4040.txt")),
        // letter: 'S' / 0x53
        #[cfg(feature = "unicode-basic-latin")]
        'S' => Some(include!("../res_rasterized_characters/4041.txt")),
        // letter: 'T' / 0x54
        #[cfg(feature = "unicode-basic-latin")]
        'T' => Some(include!("../res_rasterized_characters/4042.txt")),
        // letter: 'U' / 0x55
        #[cfg(feature = "unicode-basic-latin")]
        'U' => Some(include!("../res_rasterized_characters/4043.txt")),
        // letter: 'V' / 0x56
        #[cfg(feature = "unicode-basic-latin")]
        'V' => Some(include!("../res_rasterized_characters/4044.txt")),
        // letter: 'W' / 0x57
        #[cfg(feature = "unicode-basic-latin")]
        'W' => Some(include!("../res_rasterized_characters/4045.txt")),
        // letter: 'X' / 0x58
        #[cfg(feature = "unicode-basic-latin")]
        'X' => Some(include!("../res_rasterized_characters/4046.txt")),
        // letter: 'Y' / 0x59
        #[cfg(feature = "unicode-basic-latin")]
        'Y' => Some(include!("../res_rasterized_characters/4047.txt")),
        // letter: 'Z' / 0x5a
        #[cfg(feature = "unicode-basic-latin")]
        'Z' => Some(include!("../res_rasterized_characters/4048.txt")),
        // letter: '[' / 0x5b
        #[cfg(feature = "unicode-basic-latin")]
        '[' => Some(include!("../res_rasterized_characters/4049.txt")),
        // letter: '\' / 0x5c
        #[cfg(feature = "unicode-basic-latin")]
        '\\' => Some(include!("../res_rasterized_characters/4050.txt")),
        // letter: ']' / 0x5d
        #[cfg(feature = "unicode-basic-latin")]
        ']' => Some(include!("../res_rasterized_characters/4051.txt")),
        // letter: '^' / 0x5e
        #[cfg(feature = "unicode-basic-latin")]
        '^' => Some(include!("../res_rasterized_characters/4052.txt")),
        // letter: '_' / 0x5f
        #[cfg(feature = "unicode-basic-latin")]
        '_' => Some(include!("../res_rasterized_characters/4053.txt")),
        // letter: '`' / 0x60
        #[cfg(feature = "unicode-basic-latin")]
        '`' => Some(include!("../res_rasterized_characters/4054.txt")),
        // letter: 'a' / 0x61
        #[cfg(feature = "unicode-basic-latin")]
        'a' => Some(include!("../res_rasterized_characters/4055.txt")),
        // letter: 'b' / 0x62
        #[cfg(feature = "unicode-basic-latin")]
        'b' => Some(include!("../res_rasterized_characters/4056.txt")),
        // letter: 'c' / 0x63
        #[cfg(feature = "unicode-basic-latin")]
        'c' => Some(include!("../res_rasterized_characters/4057.txt")),
        // letter: 'd' / 0x64
        #[cfg(feature = "unicode-basic-latin")]
        'd' => Some(include!("../res_rasterized_characters/4058.txt")),
        // letter: 'e' / 0x65
        #[cfg(feature = "unicode-basic-latin")]
        'e' => Some(include!("../res_rasterized_characters/4059.txt")),
        // letter: 'f' / 0x66
        #[cfg(feature = "unicode-basic-latin")]
        'f' => Some(include!("../res_rasterized_characters/4060.txt")),
        // letter: 'g' / 0x67
        #[cfg(feature = "unicode-basic-latin")]
        'g' => Some(include!("../res_rasterized_characters/4061.txt")),
        // letter: 'h' / 0x68
        #[cfg(feature = "unicode-basic-latin")]
        'h' => Some(include!("../res_rasterized_characters/4062.txt")),
        // letter: 'i' / 0x69
        #[cfg(feature = "unicode-basic-latin")]
        'i' => Some(include!("../res_rasterized_characters/4063.txt")),
        // letter: 'j' / 0x6a
        #[cfg(feature = "unicode-basic-latin")]
        'j' => Some(include!("../res_rasterized_characters/4064.txt")),
        // letter: 'k' / 0x6b
        #[cfg(feature = "unicode-basic-latin")]
        'k' => Some(include!("../res_rasterized_characters/4065.txt")),
        // letter: 'l' / 0x6c
        #[cfg(feature = "unicode-basic-latin")]
        'l' => Some(include!("../res_rasterized_characters/4066.txt")),
        // letter: 'm' / 0x6d
        #[cfg(feature = "unicode-basic-latin")]
        'm' => Some(include!("../res_rasterized_characters/4067.txt")),
        // letter: 'n' / 0x6e
        #[cfg(feature = "unicode-basic-latin")]
        'n' => Some(include!("../res_rasterized_characters/4068.txt")),
        // letter: 'o' / 0x6f
        #[cfg(feature = "unicode-basic-latin")]
        'o' => Some(include!("../res_rasterized_characters/4069.txt")),
        // letter: 'p' / 0x70
        #[cfg(feature = "unicode-basic-latin")]
        'p' => Some(include!("../res_rasterized_characters/4070.txt")),
        // letter: 'q' / 0x71
        #[cfg(feature = "unicode-basic-latin")]
        'q' => Some(include!("../res_rasterized_characters/4071.txt")),
        // letter: 'r' / 0x72
        #[cfg(feature = "unicode-basic-latin")]
        'r' => Some(include!("../res_rasterized_characters/4072.txt")),
        // letter: 's' / 0x73
        #[cfg(feature = "unicode-basic-latin")]
        's' => Some(include!("../res_rasterized_characters/4073.txt")),
        // letter: 't' / 0x74
        #[cfg(feature = "unicode-basic-latin")]
        't' => Some(include!("../res_rasterized_characters/4074.txt")),
        // letter: 'u' / 0x75
        #[cfg(feature = "unicode-basic-latin")]
        'u' => Some(include!("../res_rasterized_characters/4075.txt")),
        // letter: 'v' / 0x76
        #[cfg(feature = "unicode-basic-latin")]
        'v' => Some(include!("../res_rasterized_characters/4076.txt")),
        // letter: 'w' / 0x77
        #[cfg(feature = "unicode-basic-latin")]
        'w' => Some(include!("../res_rasterized_characters/4077.txt")),
        // letter: 'x' / 0x78
        #[cfg(feature = "unicode-basic-latin")]
        'x' => Some(include!("../res_rasterized_characters/4078.txt")),
        // letter: 'y' / 0x79
        #[cfg(feature = "unicode-basic-latin")]
        'y' => Some(include!("../res_rasterized_characters/4079.txt")),
        // letter: 'z' / 0x7a
        #[cfg(feature = "unicode-basic-latin")]
        'z' => Some(include!("../res_rasterized_characters/4080.txt")),
        // letter: '{' / 0x7b
        #[cfg(feature = "unicode-basic-latin")]
        '{' => Some(include!("../res_rasterized_characters/4081.txt")),
        // letter: '|' / 0x7c
        #[cfg(feature = "unicode-basic-latin")]
        '|' => Some(include!("../res_rasterized_characters/4082.txt")),
        // letter: '}' / 0x7d
        #[cfg(feature = "unicode-basic-latin")]
        '}' => Some(include!("../res_rasterized_characters/4083.txt")),
        // letter: '~' / 0x7e
        #[cfg(feature = "unicode-basic-latin")]
        '~' => Some(include!("../res_rasterized_characters/4084.txt")),
        // letter: ' ' / 0xa0
        #[cfg(feature = "unicode-latin-1-supplement")]
        ' ' => Some(include!("../res_rasterized_characters/4085.txt")),
        // letter: '¡' / 0xa1
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¡' => Some(include!("../res_rasterized_characters/4086.txt")),
        // letter: '¢' / 0xa2
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¢' => Some(include!("../res_rasterized_characters/4087.txt")),
        // letter: '£' / 0xa3
        #[cfg(feature = "unicode-latin-1-supplement")]
        '£' => Some(include!("../res_rasterized_characters/4088.txt")),
        // letter: '¤' / 0xa4
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¤' => Some(include!("../res_rasterized_characters/4089.txt")),
        // letter: '¥' / 0xa5
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¥' => Some(include!("../res_rasterized_characters/4090.txt")),
        // letter: '¦' / 0xa6
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¦' => Some(include!("../res_rasterized_characters/4091.txt")),
        // letter: '§' / 0xa7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '§' => Some(include!("../res_rasterized_characters/4092.txt")),
        // letter: '¨' / 0xa8
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¨' => Some(include!("../res_rasterized_characters/4093.txt")),
        // letter: '©' / 0xa9
        #[cfg(feature = "unicode-latin-1-supplement")]
        '©' => Some(include!("../res_rasterized_characters/4094.txt")),
        // letter: 'ª' / 0xaa
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ª' => Some(include!("../res_rasterized_characters/4095.txt")),
        // letter: '«' / 0xab
        #[cfg(feature = "unicode-latin-1-supplement")]
        '«' => Some(include!("../res_rasterized_characters/4096.txt")),
        // letter: '¬' / 0xac
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¬' => Some(include!("../res_rasterized_characters/4097.txt")),
        // letter: '®' / 0xae
        #[cfg(feature = "unicode-latin-1-supplement")]
        '®' => Some(include!("../res_rasterized_characters/4098.txt")),
        // letter: '¯' / 0xaf
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¯' => Some(include!("../res_rasterized_characters/4099.txt")),
        // letter: '°' / 0xb0
        #[cfg(feature = "unicode-latin-1-supplement")]
        '°' => Some(include!("../res_rasterized_characters/4100.txt")),
        // letter: '±' / 0xb1
        #[cfg(feature = "unicode-latin-1-supplement")]
        '±' => Some(include!("../res_rasterized_characters/4101.txt")),
        // letter: '²' / 0xb2
        #[cfg(feature = "unicode-latin-1-supplement")]
        '²' => Some(include!("../res_rasterized_characters/4102.txt")),
        // letter: '³' / 0xb3
        #[cfg(feature = "unicode-latin-1-supplement")]
        '³' => Some(include!("../res_rasterized_characters/4103.txt")),
        // letter: '´' / 0xb4
        #[cfg(feature = "unicode-latin-1-supplement")]
        '´' => Some(include!("../res_rasterized_characters/4104.txt")),
        // letter: 'µ' / 0xb5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'µ' => Some(include!("../res_rasterized_characters/4105.txt")),
        // letter: '¶' / 0xb6
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¶' => Some(include!("../res_rasterized_characters/4106.txt")),
        // letter: '·' / 0xb7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '·' => Some(include!("../res_rasterized_characters/4107.txt")),
        // letter: '¸' / 0xb8
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¸' => Some(include!("../res_rasterized_characters/4108.txt")),
        // letter: '¹' / 0xb9
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¹' => Some(include!("../res_rasterized_characters/4109.txt")),
        // letter: 'º' / 0xba
        #[cfg(feature = "unicode-latin-1-supplement")]
        'º' => Some(include!("../res_rasterized_characters/4110.txt")),
        // letter: '»' / 0xbb
        #[cfg(feature = "unicode-latin-1-supplement")]
        '»' => Some(include!("../res_rasterized_characters/4111.txt")),
        // letter: '¼' / 0xbc
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¼' => Some(include!("../res_rasterized_characters/4112.txt")),
        // letter: '½' / 0xbd
        #[cfg(feature = "unicode-latin-1-supplement")]
        '½' => Some(include!("../res_rasterized_characters/4113.txt")),
        // letter: '¾' / 0xbe
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¾' => Some(include!("../res_rasterized_characters/4114.txt")),
        // letter: '¿' / 0xbf
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¿' => Some(include!("../res_rasterized_characters/4115.txt")),
        // letter: 'À' / 0xc0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'À' => Some(include!("../res_rasterized_characters/4116.txt")),
        // letter: 'Á' / 0xc1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Á' => Some(include!("../res_rasterized_characters/4117.txt")),
        // letter: 'Â' / 0xc2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Â' => Some(include!("../res_rasterized_characters/4118.txt")),
        // letter: 'Ã' / 0xc3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ã' => Some(include!("../res_rasterized_characters/4119.txt")),
        // letter: 'Ä' / 0xc4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ä' => Some(include!("../res_rasterized_characters/4120.txt")),
        // letter: 'Å' / 0xc5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Å' => Some(include!("../res_rasterized_characters/4121.txt")),
        // letter: 'Æ' / 0xc6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Æ' => Some(include!("../res_rasterized_characters/4122.txt")),
        // letter: 'Ç' / 0xc7
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ç' => Some(include!("../res_rasterized_characters/4123.txt")),
        // letter: 'È' / 0xc8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'È' => Some(include!("../res_rasterized_characters/4124.txt")),
        // letter: 'É' / 0xc9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'É' => Some(include!("../res_rasterized_characters/4125.txt")),
        // letter: 'Ê' / 0xca
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ê' => Some(include!("../res_rasterized_characters/4126.txt")),
        // letter: 'Ë' / 0xcb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ë' => Some(include!("../res_rasterized_characters/4127.txt")),
        // letter: 'Ì' / 0xcc
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ì' => Some(include!("../res_rasterized_characters/4128.txt")),
        // letter: 'Í' / 0xcd
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Í' => Some(include!("../res_rasterized_characters/4129.txt")),
        // letter: 'Î' / 0xce
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Î' => Some(include!("../res_rasterized_characters/4130.txt")),
        // letter: 'Ï' / 0xcf
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ï' => Some(include!("../res_rasterized_characters/4131.txt")),
        // letter: 'Ð' / 0xd0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ð' => Some(include!("../res_rasterized_characters/4132.txt")),
        // letter: 'Ñ' / 0xd1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ñ' => Some(include!("../res_rasterized_characters/4133.txt")),
        // letter: 'Ò' / 0xd2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ò' => Some(include!("../res_rasterized_characters/4134.txt")),
        // letter: 'Ó' / 0xd3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ó' => Some(include!("../res_rasterized_characters/4135.txt")),
        // letter: 'Ô' / 0xd4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ô' => Some(include!("../res_rasterized_characters/4136.txt")),
        // letter: 'Õ' / 0xd5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Õ' => Some(include!("../res_rasterized_characters/4137.txt")),
        // letter: 'Ö' / 0xd6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ö' => Some(include!("../res_rasterized_characters/4138.txt")),
        // letter: '×' / 0xd7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '×' => Some(include!("../res_rasterized_characters/4139.txt")),
        // letter: 'Ø' / 0xd8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ø' => Some(include!("../res_rasterized_characters/4140.txt")),
        // letter: 'Ù' / 0xd9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ù' => Some(include!("../res_rasterized_characters/4141.txt")),
        // letter: 'Ú' / 0xda
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ú' => Some(include!("../res_rasterized_characters/4142.txt")),
        // letter: 'Û' / 0xdb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Û' => Some(include!("../res_rasterized_characters/4143.txt")),
        // letter: 'Ü' / 0xdc
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ü' => Some(include!("../res_rasterized_characters/4144.txt")),
        // letter: 'Ý' / 0xdd
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ý' => Some(include!("../res_rasterized_characters/4145.txt")),
        // letter: 'Þ' / 0xde
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Þ' => Some(include!("../res_rasterized_characters/4146.txt")),
        // letter: 'ß' / 0xdf
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ß' => Some(include!("../res_rasterized_characters/4147.txt")),
        // letter: 'à' / 0xe0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'à' => Some(include!("../res_rasterized_characters/4148.txt")),
        // letter: 'á' / 0xe1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'á' => Some(include!("../res_rasterized_characters/4149.txt")),
        // letter: 'â' / 0xe2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'â' => Some(include!("../res_rasterized_characters/4150.txt")),
        // letter: 'ã' / 0xe3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ã' => Some(include!("../res_rasterized_characters/4151.txt")),
        // letter: 'ä' / 0xe4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ä' => Some(include!("../res_rasterized_characters/4152.txt")),
        // letter: 'å' / 0xe5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'å' => Some(include!("../res_rasterized_characters/4153.txt")),
        // letter: 'æ' / 0xe6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'æ' => Some(include!("../res_rasterized_characters/4154.txt")),
        // letter: 'ç' / 0xe7
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ç' => Some(include!("../res_rasterized_characters/4155.txt")),
        // letter: 'è' / 0xe8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'è' => Some(include!("../res_rasterized_characters/4156.txt")),
        // letter: 'é' / 0xe9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'é' => Some(include!("../res_rasterized_characters/4157.txt")),
        // letter: 'ê' / 0xea
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ê' => Some(include!("../res_rasterized_characters/4158.txt")),
        // letter: 'ë' / 0xeb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ë' => Some(include!("../res_rasterized_characters/4159.txt")),
        // letter: 'ì' / 0xec
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ì' => Some(include!("../res_rasterized_characters/4160.txt")),
        // letter: 'í' / 0xed
        #[cfg(feature = "unicode-latin-1-supplement")]
        'í' => Some(include!("../res_rasterized_characters/4161.txt")),
        // letter: 'î' / 0xee
        #[cfg(feature = "unicode-latin-1-supplement")]
        'î' => Some(include!("../res_rasterized_characters/4162.txt")),
        // letter: 'ï' / 0xef
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ï' => Some(include!("../res_rasterized_characters/4163.txt")),
        // letter: 'ð' / 0xf0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ð' => Some(include!("../res_rasterized_characters/4164.txt")),
        // letter: 'ñ' / 0xf1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ñ' => Some(include!("../res_rasterized_characters/4165.txt")),
        // letter: 'ò' / 0xf2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ò' => Some(include!("../res_rasterized_characters/4166.txt")),
        // letter: 'ó' / 0xf3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ó' => Some(include!("../res_rasterized_characters/4167.txt")),
        // letter: 'ô' / 0xf4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ô' => Some(include!("../res_rasterized_characters/4168.txt")),
        // letter: 'õ' / 0xf5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'õ' => Some(include!("../res_rasterized_characters/4169.txt")),
        // letter: 'ö' / 0xf6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ö' => Some(include!("../res_rasterized_characters/4170.txt")),
        // letter: '÷' / 0xf7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '÷' => Some(include!("../res_rasterized_characters/4171.txt")),
        // letter: 'ø' / 0xf8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ø' => Some(include!("../res_rasterized_characters/4172.txt")),
        // letter: 'ù' / 0xf9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ù' => Some(include!("../res_rasterized_characters/4173.txt")),
        // letter: 'ú' / 0xfa
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ú' => Some(include!("../res_rasterized_characters/4174.txt")),
        // letter: 'û' / 0xfb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'û' => Some(include!("../res_rasterized_characters/4175.txt")),
        // letter: 'ü' / 0xfc
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ü' => Some(include!("../res_rasterized_characters/4176.txt")),
        // letter: 'ý' / 0xfd
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ý' => Some(include!("../res_rasterized_characters/4177.txt")),
        // letter: 'þ' / 0xfe
        #[cfg(feature = "unicode-latin-1-supplement")]
        'þ' => Some(include!("../res_rasterized_characters/4178.txt")),
        // letter: 'ÿ' / 0xff
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ÿ' => Some(include!("../res_rasterized_characters/4179.txt")),
        _ => None,
    }
}
