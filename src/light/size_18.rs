//! Module for letters with the font weight light and size 18.
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
pub const BITMAP_HEIGHT: usize = 18;

/// The width of each bitmap character for the given font weight
/// and size. This is a few percent less than [`BITMAP_HEIGHT`],
/// because the bitmap font doesn't contain horizontal padding.
pub const BITMAP_WIDTH: usize = 10;

/// Returns the bitmap of the given character of the pre rendered
/// "Noto Sans Mono" raster for font weight light and font size 16px
#[inline]
pub const fn get_char(c: char) -> Option<&'static [&'static [u8]]> {
    match c {
        // letter: ' ' / 0x20
        #[cfg(feature = "unicode-basic-latin")]
        ' ' => Some(include!("../res_rasterized_characters/380.txt")),
        // letter: '!' / 0x21
        #[cfg(feature = "unicode-basic-latin")]
        '!' => Some(include!("../res_rasterized_characters/381.txt")),
        // letter: '"' / 0x22
        #[cfg(feature = "unicode-basic-latin")]
        '"' => Some(include!("../res_rasterized_characters/382.txt")),
        // letter: '#' / 0x23
        #[cfg(feature = "unicode-basic-latin")]
        '#' => Some(include!("../res_rasterized_characters/383.txt")),
        // letter: '$' / 0x24
        #[cfg(feature = "unicode-basic-latin")]
        '$' => Some(include!("../res_rasterized_characters/384.txt")),
        // letter: '%' / 0x25
        #[cfg(feature = "unicode-basic-latin")]
        '%' => Some(include!("../res_rasterized_characters/385.txt")),
        // letter: '&' / 0x26
        #[cfg(feature = "unicode-basic-latin")]
        '&' => Some(include!("../res_rasterized_characters/386.txt")),
        // letter: ''' / 0x27
        #[cfg(feature = "unicode-basic-latin")]
        '\'' => Some(include!("../res_rasterized_characters/387.txt")),
        // letter: '(' / 0x28
        #[cfg(feature = "unicode-basic-latin")]
        '(' => Some(include!("../res_rasterized_characters/388.txt")),
        // letter: ')' / 0x29
        #[cfg(feature = "unicode-basic-latin")]
        ')' => Some(include!("../res_rasterized_characters/389.txt")),
        // letter: '*' / 0x2a
        #[cfg(feature = "unicode-basic-latin")]
        '*' => Some(include!("../res_rasterized_characters/390.txt")),
        // letter: '+' / 0x2b
        #[cfg(feature = "unicode-basic-latin")]
        '+' => Some(include!("../res_rasterized_characters/391.txt")),
        // letter: ',' / 0x2c
        #[cfg(feature = "unicode-basic-latin")]
        ',' => Some(include!("../res_rasterized_characters/392.txt")),
        // letter: '-' / 0x2d
        #[cfg(feature = "unicode-basic-latin")]
        '-' => Some(include!("../res_rasterized_characters/393.txt")),
        // letter: '.' / 0x2e
        #[cfg(feature = "unicode-basic-latin")]
        '.' => Some(include!("../res_rasterized_characters/394.txt")),
        // letter: '/' / 0x2f
        #[cfg(feature = "unicode-basic-latin")]
        '/' => Some(include!("../res_rasterized_characters/395.txt")),
        // letter: '0' / 0x30
        #[cfg(feature = "unicode-basic-latin")]
        '0' => Some(include!("../res_rasterized_characters/396.txt")),
        // letter: '1' / 0x31
        #[cfg(feature = "unicode-basic-latin")]
        '1' => Some(include!("../res_rasterized_characters/397.txt")),
        // letter: '2' / 0x32
        #[cfg(feature = "unicode-basic-latin")]
        '2' => Some(include!("../res_rasterized_characters/398.txt")),
        // letter: '3' / 0x33
        #[cfg(feature = "unicode-basic-latin")]
        '3' => Some(include!("../res_rasterized_characters/399.txt")),
        // letter: '4' / 0x34
        #[cfg(feature = "unicode-basic-latin")]
        '4' => Some(include!("../res_rasterized_characters/400.txt")),
        // letter: '5' / 0x35
        #[cfg(feature = "unicode-basic-latin")]
        '5' => Some(include!("../res_rasterized_characters/401.txt")),
        // letter: '6' / 0x36
        #[cfg(feature = "unicode-basic-latin")]
        '6' => Some(include!("../res_rasterized_characters/402.txt")),
        // letter: '7' / 0x37
        #[cfg(feature = "unicode-basic-latin")]
        '7' => Some(include!("../res_rasterized_characters/403.txt")),
        // letter: '8' / 0x38
        #[cfg(feature = "unicode-basic-latin")]
        '8' => Some(include!("../res_rasterized_characters/404.txt")),
        // letter: '9' / 0x39
        #[cfg(feature = "unicode-basic-latin")]
        '9' => Some(include!("../res_rasterized_characters/405.txt")),
        // letter: ':' / 0x3a
        #[cfg(feature = "unicode-basic-latin")]
        ':' => Some(include!("../res_rasterized_characters/406.txt")),
        // letter: ';' / 0x3b
        #[cfg(feature = "unicode-basic-latin")]
        ';' => Some(include!("../res_rasterized_characters/407.txt")),
        // letter: '<' / 0x3c
        #[cfg(feature = "unicode-basic-latin")]
        '<' => Some(include!("../res_rasterized_characters/408.txt")),
        // letter: '=' / 0x3d
        #[cfg(feature = "unicode-basic-latin")]
        '=' => Some(include!("../res_rasterized_characters/409.txt")),
        // letter: '>' / 0x3e
        #[cfg(feature = "unicode-basic-latin")]
        '>' => Some(include!("../res_rasterized_characters/410.txt")),
        // letter: '?' / 0x3f
        #[cfg(feature = "unicode-basic-latin")]
        '?' => Some(include!("../res_rasterized_characters/411.txt")),
        // letter: '@' / 0x40
        #[cfg(feature = "unicode-basic-latin")]
        '@' => Some(include!("../res_rasterized_characters/412.txt")),
        // letter: 'A' / 0x41
        #[cfg(feature = "unicode-basic-latin")]
        'A' => Some(include!("../res_rasterized_characters/413.txt")),
        // letter: 'B' / 0x42
        #[cfg(feature = "unicode-basic-latin")]
        'B' => Some(include!("../res_rasterized_characters/414.txt")),
        // letter: 'C' / 0x43
        #[cfg(feature = "unicode-basic-latin")]
        'C' => Some(include!("../res_rasterized_characters/415.txt")),
        // letter: 'D' / 0x44
        #[cfg(feature = "unicode-basic-latin")]
        'D' => Some(include!("../res_rasterized_characters/416.txt")),
        // letter: 'E' / 0x45
        #[cfg(feature = "unicode-basic-latin")]
        'E' => Some(include!("../res_rasterized_characters/417.txt")),
        // letter: 'F' / 0x46
        #[cfg(feature = "unicode-basic-latin")]
        'F' => Some(include!("../res_rasterized_characters/418.txt")),
        // letter: 'G' / 0x47
        #[cfg(feature = "unicode-basic-latin")]
        'G' => Some(include!("../res_rasterized_characters/419.txt")),
        // letter: 'H' / 0x48
        #[cfg(feature = "unicode-basic-latin")]
        'H' => Some(include!("../res_rasterized_characters/420.txt")),
        // letter: 'I' / 0x49
        #[cfg(feature = "unicode-basic-latin")]
        'I' => Some(include!("../res_rasterized_characters/421.txt")),
        // letter: 'J' / 0x4a
        #[cfg(feature = "unicode-basic-latin")]
        'J' => Some(include!("../res_rasterized_characters/422.txt")),
        // letter: 'K' / 0x4b
        #[cfg(feature = "unicode-basic-latin")]
        'K' => Some(include!("../res_rasterized_characters/423.txt")),
        // letter: 'L' / 0x4c
        #[cfg(feature = "unicode-basic-latin")]
        'L' => Some(include!("../res_rasterized_characters/424.txt")),
        // letter: 'M' / 0x4d
        #[cfg(feature = "unicode-basic-latin")]
        'M' => Some(include!("../res_rasterized_characters/425.txt")),
        // letter: 'N' / 0x4e
        #[cfg(feature = "unicode-basic-latin")]
        'N' => Some(include!("../res_rasterized_characters/426.txt")),
        // letter: 'O' / 0x4f
        #[cfg(feature = "unicode-basic-latin")]
        'O' => Some(include!("../res_rasterized_characters/427.txt")),
        // letter: 'P' / 0x50
        #[cfg(feature = "unicode-basic-latin")]
        'P' => Some(include!("../res_rasterized_characters/428.txt")),
        // letter: 'Q' / 0x51
        #[cfg(feature = "unicode-basic-latin")]
        'Q' => Some(include!("../res_rasterized_characters/429.txt")),
        // letter: 'R' / 0x52
        #[cfg(feature = "unicode-basic-latin")]
        'R' => Some(include!("../res_rasterized_characters/430.txt")),
        // letter: 'S' / 0x53
        #[cfg(feature = "unicode-basic-latin")]
        'S' => Some(include!("../res_rasterized_characters/431.txt")),
        // letter: 'T' / 0x54
        #[cfg(feature = "unicode-basic-latin")]
        'T' => Some(include!("../res_rasterized_characters/432.txt")),
        // letter: 'U' / 0x55
        #[cfg(feature = "unicode-basic-latin")]
        'U' => Some(include!("../res_rasterized_characters/433.txt")),
        // letter: 'V' / 0x56
        #[cfg(feature = "unicode-basic-latin")]
        'V' => Some(include!("../res_rasterized_characters/434.txt")),
        // letter: 'W' / 0x57
        #[cfg(feature = "unicode-basic-latin")]
        'W' => Some(include!("../res_rasterized_characters/435.txt")),
        // letter: 'X' / 0x58
        #[cfg(feature = "unicode-basic-latin")]
        'X' => Some(include!("../res_rasterized_characters/436.txt")),
        // letter: 'Y' / 0x59
        #[cfg(feature = "unicode-basic-latin")]
        'Y' => Some(include!("../res_rasterized_characters/437.txt")),
        // letter: 'Z' / 0x5a
        #[cfg(feature = "unicode-basic-latin")]
        'Z' => Some(include!("../res_rasterized_characters/438.txt")),
        // letter: '[' / 0x5b
        #[cfg(feature = "unicode-basic-latin")]
        '[' => Some(include!("../res_rasterized_characters/439.txt")),
        // letter: '\' / 0x5c
        #[cfg(feature = "unicode-basic-latin")]
        '\\' => Some(include!("../res_rasterized_characters/440.txt")),
        // letter: ']' / 0x5d
        #[cfg(feature = "unicode-basic-latin")]
        ']' => Some(include!("../res_rasterized_characters/441.txt")),
        // letter: '^' / 0x5e
        #[cfg(feature = "unicode-basic-latin")]
        '^' => Some(include!("../res_rasterized_characters/442.txt")),
        // letter: '_' / 0x5f
        #[cfg(feature = "unicode-basic-latin")]
        '_' => Some(include!("../res_rasterized_characters/443.txt")),
        // letter: '`' / 0x60
        #[cfg(feature = "unicode-basic-latin")]
        '`' => Some(include!("../res_rasterized_characters/444.txt")),
        // letter: 'a' / 0x61
        #[cfg(feature = "unicode-basic-latin")]
        'a' => Some(include!("../res_rasterized_characters/445.txt")),
        // letter: 'b' / 0x62
        #[cfg(feature = "unicode-basic-latin")]
        'b' => Some(include!("../res_rasterized_characters/446.txt")),
        // letter: 'c' / 0x63
        #[cfg(feature = "unicode-basic-latin")]
        'c' => Some(include!("../res_rasterized_characters/447.txt")),
        // letter: 'd' / 0x64
        #[cfg(feature = "unicode-basic-latin")]
        'd' => Some(include!("../res_rasterized_characters/448.txt")),
        // letter: 'e' / 0x65
        #[cfg(feature = "unicode-basic-latin")]
        'e' => Some(include!("../res_rasterized_characters/449.txt")),
        // letter: 'f' / 0x66
        #[cfg(feature = "unicode-basic-latin")]
        'f' => Some(include!("../res_rasterized_characters/450.txt")),
        // letter: 'g' / 0x67
        #[cfg(feature = "unicode-basic-latin")]
        'g' => Some(include!("../res_rasterized_characters/451.txt")),
        // letter: 'h' / 0x68
        #[cfg(feature = "unicode-basic-latin")]
        'h' => Some(include!("../res_rasterized_characters/452.txt")),
        // letter: 'i' / 0x69
        #[cfg(feature = "unicode-basic-latin")]
        'i' => Some(include!("../res_rasterized_characters/453.txt")),
        // letter: 'j' / 0x6a
        #[cfg(feature = "unicode-basic-latin")]
        'j' => Some(include!("../res_rasterized_characters/454.txt")),
        // letter: 'k' / 0x6b
        #[cfg(feature = "unicode-basic-latin")]
        'k' => Some(include!("../res_rasterized_characters/455.txt")),
        // letter: 'l' / 0x6c
        #[cfg(feature = "unicode-basic-latin")]
        'l' => Some(include!("../res_rasterized_characters/456.txt")),
        // letter: 'm' / 0x6d
        #[cfg(feature = "unicode-basic-latin")]
        'm' => Some(include!("../res_rasterized_characters/457.txt")),
        // letter: 'n' / 0x6e
        #[cfg(feature = "unicode-basic-latin")]
        'n' => Some(include!("../res_rasterized_characters/458.txt")),
        // letter: 'o' / 0x6f
        #[cfg(feature = "unicode-basic-latin")]
        'o' => Some(include!("../res_rasterized_characters/459.txt")),
        // letter: 'p' / 0x70
        #[cfg(feature = "unicode-basic-latin")]
        'p' => Some(include!("../res_rasterized_characters/460.txt")),
        // letter: 'q' / 0x71
        #[cfg(feature = "unicode-basic-latin")]
        'q' => Some(include!("../res_rasterized_characters/461.txt")),
        // letter: 'r' / 0x72
        #[cfg(feature = "unicode-basic-latin")]
        'r' => Some(include!("../res_rasterized_characters/462.txt")),
        // letter: 's' / 0x73
        #[cfg(feature = "unicode-basic-latin")]
        's' => Some(include!("../res_rasterized_characters/463.txt")),
        // letter: 't' / 0x74
        #[cfg(feature = "unicode-basic-latin")]
        't' => Some(include!("../res_rasterized_characters/464.txt")),
        // letter: 'u' / 0x75
        #[cfg(feature = "unicode-basic-latin")]
        'u' => Some(include!("../res_rasterized_characters/465.txt")),
        // letter: 'v' / 0x76
        #[cfg(feature = "unicode-basic-latin")]
        'v' => Some(include!("../res_rasterized_characters/466.txt")),
        // letter: 'w' / 0x77
        #[cfg(feature = "unicode-basic-latin")]
        'w' => Some(include!("../res_rasterized_characters/467.txt")),
        // letter: 'x' / 0x78
        #[cfg(feature = "unicode-basic-latin")]
        'x' => Some(include!("../res_rasterized_characters/468.txt")),
        // letter: 'y' / 0x79
        #[cfg(feature = "unicode-basic-latin")]
        'y' => Some(include!("../res_rasterized_characters/469.txt")),
        // letter: 'z' / 0x7a
        #[cfg(feature = "unicode-basic-latin")]
        'z' => Some(include!("../res_rasterized_characters/470.txt")),
        // letter: '{' / 0x7b
        #[cfg(feature = "unicode-basic-latin")]
        '{' => Some(include!("../res_rasterized_characters/471.txt")),
        // letter: '|' / 0x7c
        #[cfg(feature = "unicode-basic-latin")]
        '|' => Some(include!("../res_rasterized_characters/472.txt")),
        // letter: '}' / 0x7d
        #[cfg(feature = "unicode-basic-latin")]
        '}' => Some(include!("../res_rasterized_characters/473.txt")),
        // letter: '~' / 0x7e
        #[cfg(feature = "unicode-basic-latin")]
        '~' => Some(include!("../res_rasterized_characters/474.txt")),
        // letter: ' ' / 0xa0
        #[cfg(feature = "unicode-latin-1-supplement")]
        ' ' => Some(include!("../res_rasterized_characters/475.txt")),
        // letter: '¡' / 0xa1
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¡' => Some(include!("../res_rasterized_characters/476.txt")),
        // letter: '¢' / 0xa2
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¢' => Some(include!("../res_rasterized_characters/477.txt")),
        // letter: '£' / 0xa3
        #[cfg(feature = "unicode-latin-1-supplement")]
        '£' => Some(include!("../res_rasterized_characters/478.txt")),
        // letter: '¤' / 0xa4
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¤' => Some(include!("../res_rasterized_characters/479.txt")),
        // letter: '¥' / 0xa5
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¥' => Some(include!("../res_rasterized_characters/480.txt")),
        // letter: '¦' / 0xa6
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¦' => Some(include!("../res_rasterized_characters/481.txt")),
        // letter: '§' / 0xa7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '§' => Some(include!("../res_rasterized_characters/482.txt")),
        // letter: '¨' / 0xa8
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¨' => Some(include!("../res_rasterized_characters/483.txt")),
        // letter: '©' / 0xa9
        #[cfg(feature = "unicode-latin-1-supplement")]
        '©' => Some(include!("../res_rasterized_characters/484.txt")),
        // letter: 'ª' / 0xaa
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ª' => Some(include!("../res_rasterized_characters/485.txt")),
        // letter: '«' / 0xab
        #[cfg(feature = "unicode-latin-1-supplement")]
        '«' => Some(include!("../res_rasterized_characters/486.txt")),
        // letter: '¬' / 0xac
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¬' => Some(include!("../res_rasterized_characters/487.txt")),
        // letter: '®' / 0xae
        #[cfg(feature = "unicode-latin-1-supplement")]
        '®' => Some(include!("../res_rasterized_characters/488.txt")),
        // letter: '¯' / 0xaf
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¯' => Some(include!("../res_rasterized_characters/489.txt")),
        // letter: '°' / 0xb0
        #[cfg(feature = "unicode-latin-1-supplement")]
        '°' => Some(include!("../res_rasterized_characters/490.txt")),
        // letter: '±' / 0xb1
        #[cfg(feature = "unicode-latin-1-supplement")]
        '±' => Some(include!("../res_rasterized_characters/491.txt")),
        // letter: '²' / 0xb2
        #[cfg(feature = "unicode-latin-1-supplement")]
        '²' => Some(include!("../res_rasterized_characters/492.txt")),
        // letter: '³' / 0xb3
        #[cfg(feature = "unicode-latin-1-supplement")]
        '³' => Some(include!("../res_rasterized_characters/493.txt")),
        // letter: '´' / 0xb4
        #[cfg(feature = "unicode-latin-1-supplement")]
        '´' => Some(include!("../res_rasterized_characters/494.txt")),
        // letter: 'µ' / 0xb5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'µ' => Some(include!("../res_rasterized_characters/495.txt")),
        // letter: '¶' / 0xb6
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¶' => Some(include!("../res_rasterized_characters/496.txt")),
        // letter: '·' / 0xb7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '·' => Some(include!("../res_rasterized_characters/497.txt")),
        // letter: '¸' / 0xb8
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¸' => Some(include!("../res_rasterized_characters/498.txt")),
        // letter: '¹' / 0xb9
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¹' => Some(include!("../res_rasterized_characters/499.txt")),
        // letter: 'º' / 0xba
        #[cfg(feature = "unicode-latin-1-supplement")]
        'º' => Some(include!("../res_rasterized_characters/500.txt")),
        // letter: '»' / 0xbb
        #[cfg(feature = "unicode-latin-1-supplement")]
        '»' => Some(include!("../res_rasterized_characters/501.txt")),
        // letter: '¼' / 0xbc
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¼' => Some(include!("../res_rasterized_characters/502.txt")),
        // letter: '½' / 0xbd
        #[cfg(feature = "unicode-latin-1-supplement")]
        '½' => Some(include!("../res_rasterized_characters/503.txt")),
        // letter: '¾' / 0xbe
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¾' => Some(include!("../res_rasterized_characters/504.txt")),
        // letter: '¿' / 0xbf
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¿' => Some(include!("../res_rasterized_characters/505.txt")),
        // letter: 'À' / 0xc0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'À' => Some(include!("../res_rasterized_characters/506.txt")),
        // letter: 'Á' / 0xc1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Á' => Some(include!("../res_rasterized_characters/507.txt")),
        // letter: 'Â' / 0xc2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Â' => Some(include!("../res_rasterized_characters/508.txt")),
        // letter: 'Ã' / 0xc3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ã' => Some(include!("../res_rasterized_characters/509.txt")),
        // letter: 'Ä' / 0xc4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ä' => Some(include!("../res_rasterized_characters/510.txt")),
        // letter: 'Å' / 0xc5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Å' => Some(include!("../res_rasterized_characters/511.txt")),
        // letter: 'Æ' / 0xc6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Æ' => Some(include!("../res_rasterized_characters/512.txt")),
        // letter: 'Ç' / 0xc7
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ç' => Some(include!("../res_rasterized_characters/513.txt")),
        // letter: 'È' / 0xc8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'È' => Some(include!("../res_rasterized_characters/514.txt")),
        // letter: 'É' / 0xc9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'É' => Some(include!("../res_rasterized_characters/515.txt")),
        // letter: 'Ê' / 0xca
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ê' => Some(include!("../res_rasterized_characters/516.txt")),
        // letter: 'Ë' / 0xcb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ë' => Some(include!("../res_rasterized_characters/517.txt")),
        // letter: 'Ì' / 0xcc
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ì' => Some(include!("../res_rasterized_characters/518.txt")),
        // letter: 'Í' / 0xcd
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Í' => Some(include!("../res_rasterized_characters/519.txt")),
        // letter: 'Î' / 0xce
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Î' => Some(include!("../res_rasterized_characters/520.txt")),
        // letter: 'Ï' / 0xcf
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ï' => Some(include!("../res_rasterized_characters/521.txt")),
        // letter: 'Ð' / 0xd0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ð' => Some(include!("../res_rasterized_characters/522.txt")),
        // letter: 'Ñ' / 0xd1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ñ' => Some(include!("../res_rasterized_characters/523.txt")),
        // letter: 'Ò' / 0xd2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ò' => Some(include!("../res_rasterized_characters/524.txt")),
        // letter: 'Ó' / 0xd3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ó' => Some(include!("../res_rasterized_characters/525.txt")),
        // letter: 'Ô' / 0xd4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ô' => Some(include!("../res_rasterized_characters/526.txt")),
        // letter: 'Õ' / 0xd5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Õ' => Some(include!("../res_rasterized_characters/527.txt")),
        // letter: 'Ö' / 0xd6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ö' => Some(include!("../res_rasterized_characters/528.txt")),
        // letter: '×' / 0xd7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '×' => Some(include!("../res_rasterized_characters/529.txt")),
        // letter: 'Ø' / 0xd8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ø' => Some(include!("../res_rasterized_characters/530.txt")),
        // letter: 'Ù' / 0xd9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ù' => Some(include!("../res_rasterized_characters/531.txt")),
        // letter: 'Ú' / 0xda
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ú' => Some(include!("../res_rasterized_characters/532.txt")),
        // letter: 'Û' / 0xdb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Û' => Some(include!("../res_rasterized_characters/533.txt")),
        // letter: 'Ü' / 0xdc
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ü' => Some(include!("../res_rasterized_characters/534.txt")),
        // letter: 'Ý' / 0xdd
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ý' => Some(include!("../res_rasterized_characters/535.txt")),
        // letter: 'Þ' / 0xde
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Þ' => Some(include!("../res_rasterized_characters/536.txt")),
        // letter: 'ß' / 0xdf
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ß' => Some(include!("../res_rasterized_characters/537.txt")),
        // letter: 'à' / 0xe0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'à' => Some(include!("../res_rasterized_characters/538.txt")),
        // letter: 'á' / 0xe1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'á' => Some(include!("../res_rasterized_characters/539.txt")),
        // letter: 'â' / 0xe2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'â' => Some(include!("../res_rasterized_characters/540.txt")),
        // letter: 'ã' / 0xe3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ã' => Some(include!("../res_rasterized_characters/541.txt")),
        // letter: 'ä' / 0xe4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ä' => Some(include!("../res_rasterized_characters/542.txt")),
        // letter: 'å' / 0xe5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'å' => Some(include!("../res_rasterized_characters/543.txt")),
        // letter: 'æ' / 0xe6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'æ' => Some(include!("../res_rasterized_characters/544.txt")),
        // letter: 'ç' / 0xe7
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ç' => Some(include!("../res_rasterized_characters/545.txt")),
        // letter: 'è' / 0xe8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'è' => Some(include!("../res_rasterized_characters/546.txt")),
        // letter: 'é' / 0xe9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'é' => Some(include!("../res_rasterized_characters/547.txt")),
        // letter: 'ê' / 0xea
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ê' => Some(include!("../res_rasterized_characters/548.txt")),
        // letter: 'ë' / 0xeb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ë' => Some(include!("../res_rasterized_characters/549.txt")),
        // letter: 'ì' / 0xec
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ì' => Some(include!("../res_rasterized_characters/550.txt")),
        // letter: 'í' / 0xed
        #[cfg(feature = "unicode-latin-1-supplement")]
        'í' => Some(include!("../res_rasterized_characters/551.txt")),
        // letter: 'î' / 0xee
        #[cfg(feature = "unicode-latin-1-supplement")]
        'î' => Some(include!("../res_rasterized_characters/552.txt")),
        // letter: 'ï' / 0xef
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ï' => Some(include!("../res_rasterized_characters/553.txt")),
        // letter: 'ð' / 0xf0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ð' => Some(include!("../res_rasterized_characters/554.txt")),
        // letter: 'ñ' / 0xf1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ñ' => Some(include!("../res_rasterized_characters/555.txt")),
        // letter: 'ò' / 0xf2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ò' => Some(include!("../res_rasterized_characters/556.txt")),
        // letter: 'ó' / 0xf3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ó' => Some(include!("../res_rasterized_characters/557.txt")),
        // letter: 'ô' / 0xf4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ô' => Some(include!("../res_rasterized_characters/558.txt")),
        // letter: 'õ' / 0xf5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'õ' => Some(include!("../res_rasterized_characters/559.txt")),
        // letter: 'ö' / 0xf6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ö' => Some(include!("../res_rasterized_characters/560.txt")),
        // letter: '÷' / 0xf7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '÷' => Some(include!("../res_rasterized_characters/561.txt")),
        // letter: 'ø' / 0xf8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ø' => Some(include!("../res_rasterized_characters/562.txt")),
        // letter: 'ù' / 0xf9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ù' => Some(include!("../res_rasterized_characters/563.txt")),
        // letter: 'ú' / 0xfa
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ú' => Some(include!("../res_rasterized_characters/564.txt")),
        // letter: 'û' / 0xfb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'û' => Some(include!("../res_rasterized_characters/565.txt")),
        // letter: 'ü' / 0xfc
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ü' => Some(include!("../res_rasterized_characters/566.txt")),
        // letter: 'ý' / 0xfd
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ý' => Some(include!("../res_rasterized_characters/567.txt")),
        // letter: 'þ' / 0xfe
        #[cfg(feature = "unicode-latin-1-supplement")]
        'þ' => Some(include!("../res_rasterized_characters/568.txt")),
        // letter: 'ÿ' / 0xff
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ÿ' => Some(include!("../res_rasterized_characters/569.txt")),
        _ => None,
    }
}
