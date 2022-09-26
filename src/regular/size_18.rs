//! Module for letters with the font weight regular and size 18.
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
pub const BITMAP_WIDTH: usize = 9;

/// Returns the bitmap of the given character of the pre rendered
/// "Noto Sans Mono" raster for font weight regular and font size 16px
#[inline]
pub const fn get_char(c: char) -> Option<&'static [&'static [u8]]> {
    match c {
        // letter: ' ' / 0x20
        #[cfg(feature = "unicode-basic-latin")]
        ' ' => Some(include!(
            "../res_rasterized_characters/0x20_h18_wRegular.txt"
        )),
        // letter: '!' / 0x21
        #[cfg(feature = "unicode-basic-latin")]
        '!' => Some(include!(
            "../res_rasterized_characters/0x21_h18_wRegular.txt"
        )),
        // letter: '"' / 0x22
        #[cfg(feature = "unicode-basic-latin")]
        '"' => Some(include!(
            "../res_rasterized_characters/0x22_h18_wRegular.txt"
        )),
        // letter: '#' / 0x23
        #[cfg(feature = "unicode-basic-latin")]
        '#' => Some(include!(
            "../res_rasterized_characters/0x23_h18_wRegular.txt"
        )),
        // letter: '$' / 0x24
        #[cfg(feature = "unicode-basic-latin")]
        '$' => Some(include!(
            "../res_rasterized_characters/0x24_h18_wRegular.txt"
        )),
        // letter: '%' / 0x25
        #[cfg(feature = "unicode-basic-latin")]
        '%' => Some(include!(
            "../res_rasterized_characters/0x25_h18_wRegular.txt"
        )),
        // letter: '&' / 0x26
        #[cfg(feature = "unicode-basic-latin")]
        '&' => Some(include!(
            "../res_rasterized_characters/0x26_h18_wRegular.txt"
        )),
        // letter: ''' / 0x27
        #[cfg(feature = "unicode-basic-latin")]
        '\'' => Some(include!(
            "../res_rasterized_characters/0x27_h18_wRegular.txt"
        )),
        // letter: '(' / 0x28
        #[cfg(feature = "unicode-basic-latin")]
        '(' => Some(include!(
            "../res_rasterized_characters/0x28_h18_wRegular.txt"
        )),
        // letter: ')' / 0x29
        #[cfg(feature = "unicode-basic-latin")]
        ')' => Some(include!(
            "../res_rasterized_characters/0x29_h18_wRegular.txt"
        )),
        // letter: '*' / 0x2a
        #[cfg(feature = "unicode-basic-latin")]
        '*' => Some(include!(
            "../res_rasterized_characters/0x2a_h18_wRegular.txt"
        )),
        // letter: '+' / 0x2b
        #[cfg(feature = "unicode-basic-latin")]
        '+' => Some(include!(
            "../res_rasterized_characters/0x2b_h18_wRegular.txt"
        )),
        // letter: ',' / 0x2c
        #[cfg(feature = "unicode-basic-latin")]
        ',' => Some(include!(
            "../res_rasterized_characters/0x2c_h18_wRegular.txt"
        )),
        // letter: '-' / 0x2d
        #[cfg(feature = "unicode-basic-latin")]
        '-' => Some(include!(
            "../res_rasterized_characters/0x2d_h18_wRegular.txt"
        )),
        // letter: '.' / 0x2e
        #[cfg(feature = "unicode-basic-latin")]
        '.' => Some(include!(
            "../res_rasterized_characters/0x2e_h18_wRegular.txt"
        )),
        // letter: '/' / 0x2f
        #[cfg(feature = "unicode-basic-latin")]
        '/' => Some(include!(
            "../res_rasterized_characters/0x2f_h18_wRegular.txt"
        )),
        // letter: '0' / 0x30
        #[cfg(feature = "unicode-basic-latin")]
        '0' => Some(include!(
            "../res_rasterized_characters/0x30_h18_wRegular.txt"
        )),
        // letter: '1' / 0x31
        #[cfg(feature = "unicode-basic-latin")]
        '1' => Some(include!(
            "../res_rasterized_characters/0x31_h18_wRegular.txt"
        )),
        // letter: '2' / 0x32
        #[cfg(feature = "unicode-basic-latin")]
        '2' => Some(include!(
            "../res_rasterized_characters/0x32_h18_wRegular.txt"
        )),
        // letter: '3' / 0x33
        #[cfg(feature = "unicode-basic-latin")]
        '3' => Some(include!(
            "../res_rasterized_characters/0x33_h18_wRegular.txt"
        )),
        // letter: '4' / 0x34
        #[cfg(feature = "unicode-basic-latin")]
        '4' => Some(include!(
            "../res_rasterized_characters/0x34_h18_wRegular.txt"
        )),
        // letter: '5' / 0x35
        #[cfg(feature = "unicode-basic-latin")]
        '5' => Some(include!(
            "../res_rasterized_characters/0x35_h18_wRegular.txt"
        )),
        // letter: '6' / 0x36
        #[cfg(feature = "unicode-basic-latin")]
        '6' => Some(include!(
            "../res_rasterized_characters/0x36_h18_wRegular.txt"
        )),
        // letter: '7' / 0x37
        #[cfg(feature = "unicode-basic-latin")]
        '7' => Some(include!(
            "../res_rasterized_characters/0x37_h18_wRegular.txt"
        )),
        // letter: '8' / 0x38
        #[cfg(feature = "unicode-basic-latin")]
        '8' => Some(include!(
            "../res_rasterized_characters/0x38_h18_wRegular.txt"
        )),
        // letter: '9' / 0x39
        #[cfg(feature = "unicode-basic-latin")]
        '9' => Some(include!(
            "../res_rasterized_characters/0x39_h18_wRegular.txt"
        )),
        // letter: ':' / 0x3a
        #[cfg(feature = "unicode-basic-latin")]
        ':' => Some(include!(
            "../res_rasterized_characters/0x3a_h18_wRegular.txt"
        )),
        // letter: ';' / 0x3b
        #[cfg(feature = "unicode-basic-latin")]
        ';' => Some(include!(
            "../res_rasterized_characters/0x3b_h18_wRegular.txt"
        )),
        // letter: '<' / 0x3c
        #[cfg(feature = "unicode-basic-latin")]
        '<' => Some(include!(
            "../res_rasterized_characters/0x3c_h18_wRegular.txt"
        )),
        // letter: '=' / 0x3d
        #[cfg(feature = "unicode-basic-latin")]
        '=' => Some(include!(
            "../res_rasterized_characters/0x3d_h18_wRegular.txt"
        )),
        // letter: '>' / 0x3e
        #[cfg(feature = "unicode-basic-latin")]
        '>' => Some(include!(
            "../res_rasterized_characters/0x3e_h18_wRegular.txt"
        )),
        // letter: '?' / 0x3f
        #[cfg(feature = "unicode-basic-latin")]
        '?' => Some(include!(
            "../res_rasterized_characters/0x3f_h18_wRegular.txt"
        )),
        // letter: '@' / 0x40
        #[cfg(feature = "unicode-basic-latin")]
        '@' => Some(include!(
            "../res_rasterized_characters/0x40_h18_wRegular.txt"
        )),
        // letter: 'A' / 0x41
        #[cfg(feature = "unicode-basic-latin")]
        'A' => Some(include!(
            "../res_rasterized_characters/0x41_h18_wRegular.txt"
        )),
        // letter: 'B' / 0x42
        #[cfg(feature = "unicode-basic-latin")]
        'B' => Some(include!(
            "../res_rasterized_characters/0x42_h18_wRegular.txt"
        )),
        // letter: 'C' / 0x43
        #[cfg(feature = "unicode-basic-latin")]
        'C' => Some(include!(
            "../res_rasterized_characters/0x43_h18_wRegular.txt"
        )),
        // letter: 'D' / 0x44
        #[cfg(feature = "unicode-basic-latin")]
        'D' => Some(include!(
            "../res_rasterized_characters/0x44_h18_wRegular.txt"
        )),
        // letter: 'E' / 0x45
        #[cfg(feature = "unicode-basic-latin")]
        'E' => Some(include!(
            "../res_rasterized_characters/0x45_h18_wRegular.txt"
        )),
        // letter: 'F' / 0x46
        #[cfg(feature = "unicode-basic-latin")]
        'F' => Some(include!(
            "../res_rasterized_characters/0x46_h18_wRegular.txt"
        )),
        // letter: 'G' / 0x47
        #[cfg(feature = "unicode-basic-latin")]
        'G' => Some(include!(
            "../res_rasterized_characters/0x47_h18_wRegular.txt"
        )),
        // letter: 'H' / 0x48
        #[cfg(feature = "unicode-basic-latin")]
        'H' => Some(include!(
            "../res_rasterized_characters/0x48_h18_wRegular.txt"
        )),
        // letter: 'I' / 0x49
        #[cfg(feature = "unicode-basic-latin")]
        'I' => Some(include!(
            "../res_rasterized_characters/0x49_h18_wRegular.txt"
        )),
        // letter: 'J' / 0x4a
        #[cfg(feature = "unicode-basic-latin")]
        'J' => Some(include!(
            "../res_rasterized_characters/0x4a_h18_wRegular.txt"
        )),
        // letter: 'K' / 0x4b
        #[cfg(feature = "unicode-basic-latin")]
        'K' => Some(include!(
            "../res_rasterized_characters/0x4b_h18_wRegular.txt"
        )),
        // letter: 'L' / 0x4c
        #[cfg(feature = "unicode-basic-latin")]
        'L' => Some(include!(
            "../res_rasterized_characters/0x4c_h18_wRegular.txt"
        )),
        // letter: 'M' / 0x4d
        #[cfg(feature = "unicode-basic-latin")]
        'M' => Some(include!(
            "../res_rasterized_characters/0x4d_h18_wRegular.txt"
        )),
        // letter: 'N' / 0x4e
        #[cfg(feature = "unicode-basic-latin")]
        'N' => Some(include!(
            "../res_rasterized_characters/0x4e_h18_wRegular.txt"
        )),
        // letter: 'O' / 0x4f
        #[cfg(feature = "unicode-basic-latin")]
        'O' => Some(include!(
            "../res_rasterized_characters/0x4f_h18_wRegular.txt"
        )),
        // letter: 'P' / 0x50
        #[cfg(feature = "unicode-basic-latin")]
        'P' => Some(include!(
            "../res_rasterized_characters/0x50_h18_wRegular.txt"
        )),
        // letter: 'Q' / 0x51
        #[cfg(feature = "unicode-basic-latin")]
        'Q' => Some(include!(
            "../res_rasterized_characters/0x51_h18_wRegular.txt"
        )),
        // letter: 'R' / 0x52
        #[cfg(feature = "unicode-basic-latin")]
        'R' => Some(include!(
            "../res_rasterized_characters/0x52_h18_wRegular.txt"
        )),
        // letter: 'S' / 0x53
        #[cfg(feature = "unicode-basic-latin")]
        'S' => Some(include!(
            "../res_rasterized_characters/0x53_h18_wRegular.txt"
        )),
        // letter: 'T' / 0x54
        #[cfg(feature = "unicode-basic-latin")]
        'T' => Some(include!(
            "../res_rasterized_characters/0x54_h18_wRegular.txt"
        )),
        // letter: 'U' / 0x55
        #[cfg(feature = "unicode-basic-latin")]
        'U' => Some(include!(
            "../res_rasterized_characters/0x55_h18_wRegular.txt"
        )),
        // letter: 'V' / 0x56
        #[cfg(feature = "unicode-basic-latin")]
        'V' => Some(include!(
            "../res_rasterized_characters/0x56_h18_wRegular.txt"
        )),
        // letter: 'W' / 0x57
        #[cfg(feature = "unicode-basic-latin")]
        'W' => Some(include!(
            "../res_rasterized_characters/0x57_h18_wRegular.txt"
        )),
        // letter: 'X' / 0x58
        #[cfg(feature = "unicode-basic-latin")]
        'X' => Some(include!(
            "../res_rasterized_characters/0x58_h18_wRegular.txt"
        )),
        // letter: 'Y' / 0x59
        #[cfg(feature = "unicode-basic-latin")]
        'Y' => Some(include!(
            "../res_rasterized_characters/0x59_h18_wRegular.txt"
        )),
        // letter: 'Z' / 0x5a
        #[cfg(feature = "unicode-basic-latin")]
        'Z' => Some(include!(
            "../res_rasterized_characters/0x5a_h18_wRegular.txt"
        )),
        // letter: '[' / 0x5b
        #[cfg(feature = "unicode-basic-latin")]
        '[' => Some(include!(
            "../res_rasterized_characters/0x5b_h18_wRegular.txt"
        )),
        // letter: '\' / 0x5c
        #[cfg(feature = "unicode-basic-latin")]
        '\\' => Some(include!(
            "../res_rasterized_characters/0x5c_h18_wRegular.txt"
        )),
        // letter: ']' / 0x5d
        #[cfg(feature = "unicode-basic-latin")]
        ']' => Some(include!(
            "../res_rasterized_characters/0x5d_h18_wRegular.txt"
        )),
        // letter: '^' / 0x5e
        #[cfg(feature = "unicode-basic-latin")]
        '^' => Some(include!(
            "../res_rasterized_characters/0x5e_h18_wRegular.txt"
        )),
        // letter: '_' / 0x5f
        #[cfg(feature = "unicode-basic-latin")]
        '_' => Some(include!(
            "../res_rasterized_characters/0x5f_h18_wRegular.txt"
        )),
        // letter: '`' / 0x60
        #[cfg(feature = "unicode-basic-latin")]
        '`' => Some(include!(
            "../res_rasterized_characters/0x60_h18_wRegular.txt"
        )),
        // letter: 'a' / 0x61
        #[cfg(feature = "unicode-basic-latin")]
        'a' => Some(include!(
            "../res_rasterized_characters/0x61_h18_wRegular.txt"
        )),
        // letter: 'b' / 0x62
        #[cfg(feature = "unicode-basic-latin")]
        'b' => Some(include!(
            "../res_rasterized_characters/0x62_h18_wRegular.txt"
        )),
        // letter: 'c' / 0x63
        #[cfg(feature = "unicode-basic-latin")]
        'c' => Some(include!(
            "../res_rasterized_characters/0x63_h18_wRegular.txt"
        )),
        // letter: 'd' / 0x64
        #[cfg(feature = "unicode-basic-latin")]
        'd' => Some(include!(
            "../res_rasterized_characters/0x64_h18_wRegular.txt"
        )),
        // letter: 'e' / 0x65
        #[cfg(feature = "unicode-basic-latin")]
        'e' => Some(include!(
            "../res_rasterized_characters/0x65_h18_wRegular.txt"
        )),
        // letter: 'f' / 0x66
        #[cfg(feature = "unicode-basic-latin")]
        'f' => Some(include!(
            "../res_rasterized_characters/0x66_h18_wRegular.txt"
        )),
        // letter: 'g' / 0x67
        #[cfg(feature = "unicode-basic-latin")]
        'g' => Some(include!(
            "../res_rasterized_characters/0x67_h18_wRegular.txt"
        )),
        // letter: 'h' / 0x68
        #[cfg(feature = "unicode-basic-latin")]
        'h' => Some(include!(
            "../res_rasterized_characters/0x68_h18_wRegular.txt"
        )),
        // letter: 'i' / 0x69
        #[cfg(feature = "unicode-basic-latin")]
        'i' => Some(include!(
            "../res_rasterized_characters/0x69_h18_wRegular.txt"
        )),
        // letter: 'j' / 0x6a
        #[cfg(feature = "unicode-basic-latin")]
        'j' => Some(include!(
            "../res_rasterized_characters/0x6a_h18_wRegular.txt"
        )),
        // letter: 'k' / 0x6b
        #[cfg(feature = "unicode-basic-latin")]
        'k' => Some(include!(
            "../res_rasterized_characters/0x6b_h18_wRegular.txt"
        )),
        // letter: 'l' / 0x6c
        #[cfg(feature = "unicode-basic-latin")]
        'l' => Some(include!(
            "../res_rasterized_characters/0x6c_h18_wRegular.txt"
        )),
        // letter: 'm' / 0x6d
        #[cfg(feature = "unicode-basic-latin")]
        'm' => Some(include!(
            "../res_rasterized_characters/0x6d_h18_wRegular.txt"
        )),
        // letter: 'n' / 0x6e
        #[cfg(feature = "unicode-basic-latin")]
        'n' => Some(include!(
            "../res_rasterized_characters/0x6e_h18_wRegular.txt"
        )),
        // letter: 'o' / 0x6f
        #[cfg(feature = "unicode-basic-latin")]
        'o' => Some(include!(
            "../res_rasterized_characters/0x6f_h18_wRegular.txt"
        )),
        // letter: 'p' / 0x70
        #[cfg(feature = "unicode-basic-latin")]
        'p' => Some(include!(
            "../res_rasterized_characters/0x70_h18_wRegular.txt"
        )),
        // letter: 'q' / 0x71
        #[cfg(feature = "unicode-basic-latin")]
        'q' => Some(include!(
            "../res_rasterized_characters/0x71_h18_wRegular.txt"
        )),
        // letter: 'r' / 0x72
        #[cfg(feature = "unicode-basic-latin")]
        'r' => Some(include!(
            "../res_rasterized_characters/0x72_h18_wRegular.txt"
        )),
        // letter: 's' / 0x73
        #[cfg(feature = "unicode-basic-latin")]
        's' => Some(include!(
            "../res_rasterized_characters/0x73_h18_wRegular.txt"
        )),
        // letter: 't' / 0x74
        #[cfg(feature = "unicode-basic-latin")]
        't' => Some(include!(
            "../res_rasterized_characters/0x74_h18_wRegular.txt"
        )),
        // letter: 'u' / 0x75
        #[cfg(feature = "unicode-basic-latin")]
        'u' => Some(include!(
            "../res_rasterized_characters/0x75_h18_wRegular.txt"
        )),
        // letter: 'v' / 0x76
        #[cfg(feature = "unicode-basic-latin")]
        'v' => Some(include!(
            "../res_rasterized_characters/0x76_h18_wRegular.txt"
        )),
        // letter: 'w' / 0x77
        #[cfg(feature = "unicode-basic-latin")]
        'w' => Some(include!(
            "../res_rasterized_characters/0x77_h18_wRegular.txt"
        )),
        // letter: 'x' / 0x78
        #[cfg(feature = "unicode-basic-latin")]
        'x' => Some(include!(
            "../res_rasterized_characters/0x78_h18_wRegular.txt"
        )),
        // letter: 'y' / 0x79
        #[cfg(feature = "unicode-basic-latin")]
        'y' => Some(include!(
            "../res_rasterized_characters/0x79_h18_wRegular.txt"
        )),
        // letter: 'z' / 0x7a
        #[cfg(feature = "unicode-basic-latin")]
        'z' => Some(include!(
            "../res_rasterized_characters/0x7a_h18_wRegular.txt"
        )),
        // letter: '{' / 0x7b
        #[cfg(feature = "unicode-basic-latin")]
        '{' => Some(include!(
            "../res_rasterized_characters/0x7b_h18_wRegular.txt"
        )),
        // letter: '|' / 0x7c
        #[cfg(feature = "unicode-basic-latin")]
        '|' => Some(include!(
            "../res_rasterized_characters/0x7c_h18_wRegular.txt"
        )),
        // letter: '}' / 0x7d
        #[cfg(feature = "unicode-basic-latin")]
        '}' => Some(include!(
            "../res_rasterized_characters/0x7d_h18_wRegular.txt"
        )),
        // letter: '~' / 0x7e
        #[cfg(feature = "unicode-basic-latin")]
        '~' => Some(include!(
            "../res_rasterized_characters/0x7e_h18_wRegular.txt"
        )),
        // letter: ' ' / 0xa0
        #[cfg(feature = "unicode-latin-1-supplement")]
        ' ' => Some(include!(
            "../res_rasterized_characters/0xa0_h18_wRegular.txt"
        )),
        // letter: '¡' / 0xa1
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¡' => Some(include!(
            "../res_rasterized_characters/0xa1_h18_wRegular.txt"
        )),
        // letter: '¢' / 0xa2
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¢' => Some(include!(
            "../res_rasterized_characters/0xa2_h18_wRegular.txt"
        )),
        // letter: '£' / 0xa3
        #[cfg(feature = "unicode-latin-1-supplement")]
        '£' => Some(include!(
            "../res_rasterized_characters/0xa3_h18_wRegular.txt"
        )),
        // letter: '¤' / 0xa4
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¤' => Some(include!(
            "../res_rasterized_characters/0xa4_h18_wRegular.txt"
        )),
        // letter: '¥' / 0xa5
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¥' => Some(include!(
            "../res_rasterized_characters/0xa5_h18_wRegular.txt"
        )),
        // letter: '¦' / 0xa6
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¦' => Some(include!(
            "../res_rasterized_characters/0xa6_h18_wRegular.txt"
        )),
        // letter: '§' / 0xa7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '§' => Some(include!(
            "../res_rasterized_characters/0xa7_h18_wRegular.txt"
        )),
        // letter: '¨' / 0xa8
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¨' => Some(include!(
            "../res_rasterized_characters/0xa8_h18_wRegular.txt"
        )),
        // letter: '©' / 0xa9
        #[cfg(feature = "unicode-latin-1-supplement")]
        '©' => Some(include!(
            "../res_rasterized_characters/0xa9_h18_wRegular.txt"
        )),
        // letter: 'ª' / 0xaa
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ª' => Some(include!(
            "../res_rasterized_characters/0xaa_h18_wRegular.txt"
        )),
        // letter: '«' / 0xab
        #[cfg(feature = "unicode-latin-1-supplement")]
        '«' => Some(include!(
            "../res_rasterized_characters/0xab_h18_wRegular.txt"
        )),
        // letter: '¬' / 0xac
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¬' => Some(include!(
            "../res_rasterized_characters/0xac_h18_wRegular.txt"
        )),
        // letter: '®' / 0xae
        #[cfg(feature = "unicode-latin-1-supplement")]
        '®' => Some(include!(
            "../res_rasterized_characters/0xae_h18_wRegular.txt"
        )),
        // letter: '¯' / 0xaf
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¯' => Some(include!(
            "../res_rasterized_characters/0xaf_h18_wRegular.txt"
        )),
        // letter: '°' / 0xb0
        #[cfg(feature = "unicode-latin-1-supplement")]
        '°' => Some(include!(
            "../res_rasterized_characters/0xb0_h18_wRegular.txt"
        )),
        // letter: '±' / 0xb1
        #[cfg(feature = "unicode-latin-1-supplement")]
        '±' => Some(include!(
            "../res_rasterized_characters/0xb1_h18_wRegular.txt"
        )),
        // letter: '²' / 0xb2
        #[cfg(feature = "unicode-latin-1-supplement")]
        '²' => Some(include!(
            "../res_rasterized_characters/0xb2_h18_wRegular.txt"
        )),
        // letter: '³' / 0xb3
        #[cfg(feature = "unicode-latin-1-supplement")]
        '³' => Some(include!(
            "../res_rasterized_characters/0xb3_h18_wRegular.txt"
        )),
        // letter: '´' / 0xb4
        #[cfg(feature = "unicode-latin-1-supplement")]
        '´' => Some(include!(
            "../res_rasterized_characters/0xb4_h18_wRegular.txt"
        )),
        // letter: 'µ' / 0xb5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'µ' => Some(include!(
            "../res_rasterized_characters/0xb5_h18_wRegular.txt"
        )),
        // letter: '¶' / 0xb6
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¶' => Some(include!(
            "../res_rasterized_characters/0xb6_h18_wRegular.txt"
        )),
        // letter: '·' / 0xb7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '·' => Some(include!(
            "../res_rasterized_characters/0xb7_h18_wRegular.txt"
        )),
        // letter: '¸' / 0xb8
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¸' => Some(include!(
            "../res_rasterized_characters/0xb8_h18_wRegular.txt"
        )),
        // letter: '¹' / 0xb9
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¹' => Some(include!(
            "../res_rasterized_characters/0xb9_h18_wRegular.txt"
        )),
        // letter: 'º' / 0xba
        #[cfg(feature = "unicode-latin-1-supplement")]
        'º' => Some(include!(
            "../res_rasterized_characters/0xba_h18_wRegular.txt"
        )),
        // letter: '»' / 0xbb
        #[cfg(feature = "unicode-latin-1-supplement")]
        '»' => Some(include!(
            "../res_rasterized_characters/0xbb_h18_wRegular.txt"
        )),
        // letter: '¼' / 0xbc
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¼' => Some(include!(
            "../res_rasterized_characters/0xbc_h18_wRegular.txt"
        )),
        // letter: '½' / 0xbd
        #[cfg(feature = "unicode-latin-1-supplement")]
        '½' => Some(include!(
            "../res_rasterized_characters/0xbd_h18_wRegular.txt"
        )),
        // letter: '¾' / 0xbe
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¾' => Some(include!(
            "../res_rasterized_characters/0xbe_h18_wRegular.txt"
        )),
        // letter: '¿' / 0xbf
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¿' => Some(include!(
            "../res_rasterized_characters/0xbf_h18_wRegular.txt"
        )),
        // letter: 'À' / 0xc0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'À' => Some(include!(
            "../res_rasterized_characters/0xc0_h18_wRegular.txt"
        )),
        // letter: 'Á' / 0xc1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Á' => Some(include!(
            "../res_rasterized_characters/0xc1_h18_wRegular.txt"
        )),
        // letter: 'Â' / 0xc2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Â' => Some(include!(
            "../res_rasterized_characters/0xc2_h18_wRegular.txt"
        )),
        // letter: 'Ã' / 0xc3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ã' => Some(include!(
            "../res_rasterized_characters/0xc3_h18_wRegular.txt"
        )),
        // letter: 'Ä' / 0xc4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ä' => Some(include!(
            "../res_rasterized_characters/0xc4_h18_wRegular.txt"
        )),
        // letter: 'Å' / 0xc5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Å' => Some(include!(
            "../res_rasterized_characters/0xc5_h18_wRegular.txt"
        )),
        // letter: 'Æ' / 0xc6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Æ' => Some(include!(
            "../res_rasterized_characters/0xc6_h18_wRegular.txt"
        )),
        // letter: 'Ç' / 0xc7
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ç' => Some(include!(
            "../res_rasterized_characters/0xc7_h18_wRegular.txt"
        )),
        // letter: 'È' / 0xc8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'È' => Some(include!(
            "../res_rasterized_characters/0xc8_h18_wRegular.txt"
        )),
        // letter: 'É' / 0xc9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'É' => Some(include!(
            "../res_rasterized_characters/0xc9_h18_wRegular.txt"
        )),
        // letter: 'Ê' / 0xca
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ê' => Some(include!(
            "../res_rasterized_characters/0xca_h18_wRegular.txt"
        )),
        // letter: 'Ë' / 0xcb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ë' => Some(include!(
            "../res_rasterized_characters/0xcb_h18_wRegular.txt"
        )),
        // letter: 'Ì' / 0xcc
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ì' => Some(include!(
            "../res_rasterized_characters/0xcc_h18_wRegular.txt"
        )),
        // letter: 'Í' / 0xcd
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Í' => Some(include!(
            "../res_rasterized_characters/0xcd_h18_wRegular.txt"
        )),
        // letter: 'Î' / 0xce
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Î' => Some(include!(
            "../res_rasterized_characters/0xce_h18_wRegular.txt"
        )),
        // letter: 'Ï' / 0xcf
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ï' => Some(include!(
            "../res_rasterized_characters/0xcf_h18_wRegular.txt"
        )),
        // letter: 'Ð' / 0xd0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ð' => Some(include!(
            "../res_rasterized_characters/0xd0_h18_wRegular.txt"
        )),
        // letter: 'Ñ' / 0xd1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ñ' => Some(include!(
            "../res_rasterized_characters/0xd1_h18_wRegular.txt"
        )),
        // letter: 'Ò' / 0xd2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ò' => Some(include!(
            "../res_rasterized_characters/0xd2_h18_wRegular.txt"
        )),
        // letter: 'Ó' / 0xd3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ó' => Some(include!(
            "../res_rasterized_characters/0xd3_h18_wRegular.txt"
        )),
        // letter: 'Ô' / 0xd4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ô' => Some(include!(
            "../res_rasterized_characters/0xd4_h18_wRegular.txt"
        )),
        // letter: 'Õ' / 0xd5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Õ' => Some(include!(
            "../res_rasterized_characters/0xd5_h18_wRegular.txt"
        )),
        // letter: 'Ö' / 0xd6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ö' => Some(include!(
            "../res_rasterized_characters/0xd6_h18_wRegular.txt"
        )),
        // letter: '×' / 0xd7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '×' => Some(include!(
            "../res_rasterized_characters/0xd7_h18_wRegular.txt"
        )),
        // letter: 'Ø' / 0xd8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ø' => Some(include!(
            "../res_rasterized_characters/0xd8_h18_wRegular.txt"
        )),
        // letter: 'Ù' / 0xd9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ù' => Some(include!(
            "../res_rasterized_characters/0xd9_h18_wRegular.txt"
        )),
        // letter: 'Ú' / 0xda
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ú' => Some(include!(
            "../res_rasterized_characters/0xda_h18_wRegular.txt"
        )),
        // letter: 'Û' / 0xdb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Û' => Some(include!(
            "../res_rasterized_characters/0xdb_h18_wRegular.txt"
        )),
        // letter: 'Ü' / 0xdc
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ü' => Some(include!(
            "../res_rasterized_characters/0xdc_h18_wRegular.txt"
        )),
        // letter: 'Ý' / 0xdd
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ý' => Some(include!(
            "../res_rasterized_characters/0xdd_h18_wRegular.txt"
        )),
        // letter: 'Þ' / 0xde
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Þ' => Some(include!(
            "../res_rasterized_characters/0xde_h18_wRegular.txt"
        )),
        // letter: 'ß' / 0xdf
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ß' => Some(include!(
            "../res_rasterized_characters/0xdf_h18_wRegular.txt"
        )),
        // letter: 'à' / 0xe0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'à' => Some(include!(
            "../res_rasterized_characters/0xe0_h18_wRegular.txt"
        )),
        // letter: 'á' / 0xe1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'á' => Some(include!(
            "../res_rasterized_characters/0xe1_h18_wRegular.txt"
        )),
        // letter: 'â' / 0xe2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'â' => Some(include!(
            "../res_rasterized_characters/0xe2_h18_wRegular.txt"
        )),
        // letter: 'ã' / 0xe3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ã' => Some(include!(
            "../res_rasterized_characters/0xe3_h18_wRegular.txt"
        )),
        // letter: 'ä' / 0xe4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ä' => Some(include!(
            "../res_rasterized_characters/0xe4_h18_wRegular.txt"
        )),
        // letter: 'å' / 0xe5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'å' => Some(include!(
            "../res_rasterized_characters/0xe5_h18_wRegular.txt"
        )),
        // letter: 'æ' / 0xe6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'æ' => Some(include!(
            "../res_rasterized_characters/0xe6_h18_wRegular.txt"
        )),
        // letter: 'ç' / 0xe7
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ç' => Some(include!(
            "../res_rasterized_characters/0xe7_h18_wRegular.txt"
        )),
        // letter: 'è' / 0xe8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'è' => Some(include!(
            "../res_rasterized_characters/0xe8_h18_wRegular.txt"
        )),
        // letter: 'é' / 0xe9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'é' => Some(include!(
            "../res_rasterized_characters/0xe9_h18_wRegular.txt"
        )),
        // letter: 'ê' / 0xea
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ê' => Some(include!(
            "../res_rasterized_characters/0xea_h18_wRegular.txt"
        )),
        // letter: 'ë' / 0xeb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ë' => Some(include!(
            "../res_rasterized_characters/0xeb_h18_wRegular.txt"
        )),
        // letter: 'ì' / 0xec
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ì' => Some(include!(
            "../res_rasterized_characters/0xec_h18_wRegular.txt"
        )),
        // letter: 'í' / 0xed
        #[cfg(feature = "unicode-latin-1-supplement")]
        'í' => Some(include!(
            "../res_rasterized_characters/0xed_h18_wRegular.txt"
        )),
        // letter: 'î' / 0xee
        #[cfg(feature = "unicode-latin-1-supplement")]
        'î' => Some(include!(
            "../res_rasterized_characters/0xee_h18_wRegular.txt"
        )),
        // letter: 'ï' / 0xef
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ï' => Some(include!(
            "../res_rasterized_characters/0xef_h18_wRegular.txt"
        )),
        // letter: 'ð' / 0xf0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ð' => Some(include!(
            "../res_rasterized_characters/0xf0_h18_wRegular.txt"
        )),
        // letter: 'ñ' / 0xf1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ñ' => Some(include!(
            "../res_rasterized_characters/0xf1_h18_wRegular.txt"
        )),
        // letter: 'ò' / 0xf2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ò' => Some(include!(
            "../res_rasterized_characters/0xf2_h18_wRegular.txt"
        )),
        // letter: 'ó' / 0xf3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ó' => Some(include!(
            "../res_rasterized_characters/0xf3_h18_wRegular.txt"
        )),
        // letter: 'ô' / 0xf4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ô' => Some(include!(
            "../res_rasterized_characters/0xf4_h18_wRegular.txt"
        )),
        // letter: 'õ' / 0xf5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'õ' => Some(include!(
            "../res_rasterized_characters/0xf5_h18_wRegular.txt"
        )),
        // letter: 'ö' / 0xf6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ö' => Some(include!(
            "../res_rasterized_characters/0xf6_h18_wRegular.txt"
        )),
        // letter: '÷' / 0xf7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '÷' => Some(include!(
            "../res_rasterized_characters/0xf7_h18_wRegular.txt"
        )),
        // letter: 'ø' / 0xf8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ø' => Some(include!(
            "../res_rasterized_characters/0xf8_h18_wRegular.txt"
        )),
        // letter: 'ù' / 0xf9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ù' => Some(include!(
            "../res_rasterized_characters/0xf9_h18_wRegular.txt"
        )),
        // letter: 'ú' / 0xfa
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ú' => Some(include!(
            "../res_rasterized_characters/0xfa_h18_wRegular.txt"
        )),
        // letter: 'û' / 0xfb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'û' => Some(include!(
            "../res_rasterized_characters/0xfb_h18_wRegular.txt"
        )),
        // letter: 'ü' / 0xfc
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ü' => Some(include!(
            "../res_rasterized_characters/0xfc_h18_wRegular.txt"
        )),
        // letter: 'ý' / 0xfd
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ý' => Some(include!(
            "../res_rasterized_characters/0xfd_h18_wRegular.txt"
        )),
        // letter: 'þ' / 0xfe
        #[cfg(feature = "unicode-latin-1-supplement")]
        'þ' => Some(include!(
            "../res_rasterized_characters/0xfe_h18_wRegular.txt"
        )),
        // letter: 'ÿ' / 0xff
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ÿ' => Some(include!(
            "../res_rasterized_characters/0xff_h18_wRegular.txt"
        )),
        _ => None,
    }
}
