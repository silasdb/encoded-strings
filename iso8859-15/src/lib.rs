//! # ISO8859-15 String Library
//!
//! This crate provides string and character types that are encoded in ISO8859-15.

use std::fmt;

/// A single ISO8859-10 character.
///
/// # Validity
/// A `IsoLatin6Char` is valid if it is a valid well defined ISO8859-10 character or ASCII control
/// codes.
///
/// ## Why ASCII control codes are valid?
/// Although ISO8859-10 does not define ASCII control codes values (`0x00` to `0x1F`), we consider
/// them valid for convenience.
///
/// Since these code values are considered undefined by the standard, the decision on what to do
/// with them is implementation defined. Its commom to implement this standard considering those
/// code values like we do.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct IsoLatin9Char(u8);

// Public API
impl IsoLatin9Char {
    /// Returns `true` if this character has the `Alphabetic` property.
    ///
    /// `Alphabetic` is described in Chapter 4 (Character Properties) of the [Unicode Standard] and
    /// specified in the [Unicode Character Database][ucd] [`DerivedCoreProperties.txt`].
    ///
    /// Althought this type is not an Unicode, we use the same database to get the property for the
    /// character symbols.
    ///
    /// [Unicode Standard]: https://www.unicode.org/versions/latest/
    /// [ucd]: https://www.unicode.org/reports/tr44/
    /// [`DerivedCoreProperties.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// TODO
    pub fn is_alphabetic(&self) -> bool {
        todo!()
    }

    /// Returns `true` if this character satisfies either [`is_alphabetic`] or [`is_numeric`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// TODO
    pub fn is_alphanumeric(&self) -> bool {
        self.is_alphabetic() || self.is_numeric()
    }

    /// Returns `true` if this character has the general category for control codes.
    ///
    /// Control codes (code points with the general category of `Cc`) are described in Chapter 4
    /// (Character Properties) of the [Unicode Standard] and specified in the [Unicode Character
    /// Database][ucd] [`UnicodeData.txt`].
    ///
    /// Althought this type is not an Unicode, we use the same database to get the property for the
    /// character symbols.
    ///
    /// [Unicode Standard]: https://www.unicode.org/versions/latest/
    /// [ucd]: https://www.unicode.org/reports/tr44/
    /// [`UnicodeData.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// TODO
    pub fn is_control(&self) -> bool {
        todo!()
    }

    /// Checks if a `char` is a digit in the given radix.
    ///
    /// A 'radix' here is sometimes also called a 'base'. A radix of two
    /// indicates a binary number, a radix of ten, decimal, and a radix of
    /// sixteen, hexadecimal, to give some common values. Arbitrary
    /// radices are supported.
    ///
    /// Compared to [`is_numeric()`], this function only recognizes the characters
    /// `0-9`, `a-z` and `A-Z`.
    ///
    /// 'Digit' is defined to be only the following characters:
    ///
    /// * `0-9`
    /// * `a-z`
    /// * `A-Z`
    ///
    /// For a more comprehensive understanding of 'digit', see [`is_numeric()`].
    ///
    /// [`is_numeric()`]: #method.is_numeric
    ///
    /// # Panics
    ///
    /// Panics if given a radix larger than 36.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// TODO
    /// ```
    ///
    /// Passing a large radix, causing a panic:
    ///
    /// ```should_panic
    /// TODO
    /// ```
    pub fn is_digit(&self, radix: u8) -> bool {
        todo!()
    }

    /// Returns `true` if this character has one of the general categories for numbers.
    ///
    /// The general categories for numbers (`Nd` for decimal digits, `Nl` for letter-like numeric
    /// characters, and `No` for other numeric characters) are specified in the [Unicode Character
    /// Database][ucd] [`UnicodeData.txt`].
    ///
    /// Althought this type is not an Unicode, we use the same database to get the property for the
    /// character symbols.
    ///
    /// [Unicode Standard]: https://www.unicode.org/versions/latest/
    /// [ucd]: https://www.unicode.org/reports/tr44/
    /// [`UnicodeData.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// TODO
    /// ```
    pub fn is_numeric(&self) -> bool {
        match self.0 {
            0x30..=0x39 |      // between 0 to 9
            0xB2..=0xB3 |  // between ² to ³
            0xB9 => true,    // only ¹
            _ => false
        }
    }

    /// Returns `true` if this character has the `White_Space` property.
    ///
    /// `White_Space` is specified in the [Unicode Character Database][ucd] [`PropList.txt`].
    ///
    /// Althought this type is not an Unicode, we use the same database to get the property for the
    /// character symbols.
    ///
    /// [ucd]: https://www.unicode.org/reports/tr44/
    /// [`PropList.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// TODO
    /// ```
    pub fn is_whitespace(&self) -> bool {
        todo!()
    }

    /// Returns `true` if this character has the `Lowercase` property.
    ///
    /// `Lowercase` is described in Chapter 4 (Character Properties) of the [Unicode Standard] and
    /// specified in the [Unicode Character Database][ucd] [`DerivedCoreProperties.txt`].
    ///
    /// Althought this type is not an Unicode, we use the same database to get the property for the
    /// character symbols.
    ///
    /// [Unicode Standard]: https://www.unicode.org/versions/latest/
    /// [ucd]: https://www.unicode.org/reports/tr44/
    /// [`DerivedCoreProperties.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// TODO
    /// ```
    pub fn is_lowercase(&self) -> bool {
        todo!()
    }

    /// Returns `true` if this character has the `Uppercase` property.
    ///
    /// `Uppercase` is described in Chapter 4 (Character Properties) of the [Unicode Standard] and
    /// specified in the [Unicode Character Database][ucd] [`DerivedCoreProperties.txt`].
    ///
    /// Althought this type is not an Unicode, we use the same database to get the property for the
    /// character symbols.
    ///
    /// [Unicode Standard]: https://www.unicode.org/versions/latest/
    /// [ucd]: https://www.unicode.org/reports/tr44/
    /// [`DerivedCoreProperties.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// TODO
    /// ```
    pub fn is_uppercase(&self) -> bool {
        todo!()
    }
}

// Public API related to ASCII
impl IsoLatin9Char {
    /// Checks if the value is within the ASCII range.
    ///
    /// # Examples
    ///
    /// ```
    /// TODO
    /// ```
    pub fn is_ascii(&self) -> bool {
        self.0 <= 0x7F
    }
}

impl fmt::Debug for IsoLatin9Char {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Display for IsoLatin9Char {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::LowerHex for IsoLatin9Char {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::UpperHex for IsoLatin9Char {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl TryFrom<u8> for IsoLatin9Char {
    type Error = IsoLatin9CharError;

    #[inline]
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<IsoLatin9Char> for u8 {
    #[inline]
    fn from(char: IsoLatin9Char) -> u8 {
        todo!()
    }
}

impl TryFrom<char> for IsoLatin9Char {
    type Error = IsoLatin9CharError;

    #[inline]
    fn try_from(char: char) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<IsoLatin9Char> for char {
    #[inline]
    fn from(char: IsoLatin9Char) -> Self {
        todo!()
    }
}

/// Error type to represent possible reasons for a byte not being a valid [`IsoLatin6Char`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IsoLatin9CharError {
    /// The byte is not defined as a specific character in ISO8859-10 and it's not ASCII control
    /// codes.
    Undefined,
    /// The byte contains a invalid value.
    Invalid,
}
#[cfg(test)]
mod api_tests {
    use super::*;

    #[test]
    fn is_alphabetic() {
        todo!()
    }

    #[test]
    fn is_control() {
        for byte in 0x00..=0x1F {
            assert!(IsoLatin9Char(byte).is_control());
        }
        for byte in 0x20..=0xFF {
            assert!(!IsoLatin9Char(byte).is_control());
        }
    }

    #[test]
    fn is_digit() {
        assert!(IsoLatin9Char(b'0').is_digit(10));
        assert!(IsoLatin9Char(b'1').is_digit(2));
        assert!(IsoLatin9Char(b'2').is_digit(3));
        assert!(IsoLatin9Char(b'9').is_digit(10));
        assert!(IsoLatin9Char(b'a').is_digit(16),);
        assert!(IsoLatin9Char(b'A').is_digit(16),);
        assert!(IsoLatin9Char(b'b').is_digit(16),);
        assert!(IsoLatin9Char(b'B').is_digit(16),);
        assert!(IsoLatin9Char(b'A').is_digit(36),);
        assert!(IsoLatin9Char(b'z').is_digit(36),);
        assert!(IsoLatin9Char(b'Z').is_digit(36),);
        assert!(!IsoLatin9Char(b'[').is_digit(36));
        assert!(!IsoLatin9Char(b'`').is_digit(36));
        assert!(!IsoLatin9Char(b'{').is_digit(36));
        assert!(!IsoLatin9Char(b'$').is_digit(36));
        assert!(!IsoLatin9Char(b'@').is_digit(16));
        assert!(!IsoLatin9Char(b'G').is_digit(16));
        assert!(!IsoLatin9Char(b'g').is_digit(16));
        assert!(!IsoLatin9Char(b' ').is_digit(10));
        assert!(!IsoLatin9Char(b'/').is_digit(10));
        assert!(!IsoLatin9Char(b':').is_digit(10));
        assert!(!IsoLatin9Char(b':').is_digit(11));
    }

    #[test]
    fn is_numeric() {
        let numerics: Vec<u8> = [
            [0x30..=0x39, 0xB2..=0xB3]
                .into_iter()
                .map(|range| range.collect::<Vec<_>>())
                .flatten()
                .collect(),
            vec![0xB9],
        ]
        .concat();
        for byte in 0x00..=0xFF {
            if numerics.contains(&byte) {
                assert!(IsoLatin9Char(byte).is_numeric());
            } else {
                assert!(!IsoLatin9Char(byte).is_numeric());
            }
        }
    }

    #[test]
    fn is_whitespace() {
        assert!(IsoLatin9Char(b' ').is_whitespace());
        assert!(IsoLatin9Char(b'\t').is_whitespace());
        assert!(IsoLatin9Char(b'\n').is_whitespace());
        assert!(!IsoLatin9Char(b'a').is_whitespace());
        assert!(!IsoLatin9Char(b'_').is_whitespace());
        assert!(!IsoLatin9Char(b'\0').is_whitespace());
    }

    #[test]
    fn is_uppercase() {
        assert!(IsoLatin9Char(b'A').is_uppercase());
        assert!(IsoLatin9Char(b'Z').is_uppercase());
        assert!(!IsoLatin9Char(b'a').is_uppercase());
        assert!(!IsoLatin9Char(b'z').is_uppercase());
        assert!(!IsoLatin9Char(b'0').is_uppercase());
        assert!(!IsoLatin9Char(b'9').is_uppercase());
        assert!(!IsoLatin9Char(b'_').is_uppercase());
        assert!(!IsoLatin9Char(b'\0').is_uppercase());
    }

    #[test]
    fn is_lowercase() {
        assert!(IsoLatin9Char(b'a').is_lowercase());
        assert!(IsoLatin9Char(b'z').is_lowercase());
        assert!(!IsoLatin9Char(b'A').is_lowercase());
        assert!(!IsoLatin9Char(b'Z').is_lowercase());
        assert!(!IsoLatin9Char(b'0').is_lowercase());
        assert!(!IsoLatin9Char(b'9').is_lowercase());
        assert!(!IsoLatin9Char(b'_').is_lowercase());
        assert!(!IsoLatin9Char(b'\0').is_lowercase());
    }
}

#[cfg(test)]
mod trait_tests {
    use super::*;

    static LAST_PART_OF_ISO8859: [char; 96] = [
        '\u{A0}', 'Ą', 'Ē', 'Ģ', 'Ī', 'Ĩ', 'Ķ', '§', 'Ļ', 'Đ', 'Š', 'Ŧ', 'Ž', '\u{AD}', 'Ū', 'Ŋ',
        '°', 'ą', 'ē', 'ģ', 'ī', 'ĩ', 'ķ', '·', 'ļ', 'đ', 'š', 'ŧ', 'ž', '―', 'ū', 'ŋ', 'Ā', 'Á',
        'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Į', 'Č', 'É', 'Ę', 'Ë', 'Ė', 'Í', 'Î', 'Ï', 'Ð', 'Ņ', 'Ō', 'Ó',
        'Ô', 'Õ', 'Ö', 'Ũ', 'Ø', 'Ų', 'Ú', 'Û', 'Ü', 'Ý', 'Þ', 'ß', 'ā', 'á', 'â', 'ã', 'ä', 'å',
        'æ', 'į', 'č', 'é', 'ę', 'ë', 'ė', 'í', 'î', 'ï', 'ð', 'ņ', 'ō', 'ó', 'ô', 'õ', 'ö', 'ũ',
        'ø', 'ų', 'ú', 'û', 'ü', 'ý', 'þ', 'ĸ',
    ];

    #[test]
    fn debug() {
        let upcase_a = IsoLatin9Char(0x41);
        assert_eq!(format!("{:?}", upcase_a), "'A'");

        let upcase_ash = IsoLatin9Char(0xC6);
        assert_eq!(format!("{:?}", upcase_ash), "'Æ'");

        todo!()
    }

    #[test]
    fn display() {
        let upcase_a = IsoLatin9Char(0x41);
        assert_eq!(format!("{:?}", upcase_a), "A");

        let upcase_ash = IsoLatin9Char(0xC6);
        assert_eq!(format!("{:?}", upcase_ash), "Æ");

        todo!()
    }

    #[test]
    fn lowerhex() {
        for byte in 0x00..=0xFF {
            let char = IsoLatin9Char(byte);
            assert_eq!(format!("{:x}", char), format!("{:x}", byte));
        }
    }

    #[test]
    fn upperhex() {
        for byte in 0x00..=0xFF {
            let char = IsoLatin9Char(byte);
            assert_eq!(format!("{:X}", char), format!("{:X}", byte));
        }
    }

    #[test]
    fn from_self_to_u8() {
        for byte in 0x00..=0xFF {
            let char = IsoLatin9Char(byte);
            assert_eq!(u8::from(char), byte);
        }
    }

    #[test]
    fn from_self_to_char() {
        todo!()
    }

    #[test]
    fn try_from_u8_to_self() {
        for byte in 0x00..=0x7F {
            assert!(IsoLatin9Char::try_from(byte).is_ok(), "0x{byte:x}");
        }

        for byte in 0x80..=0x9F {
            assert_eq!(
                IsoLatin9Char::try_from(byte),
                Err(IsoLatin9CharError::Undefined),
                "{byte:x}"
            );
        }

        todo!()
    }

    #[test]
    fn try_from_char_to_self() {
        for char in '\u{00}'..='\u{7F}' {
            assert!(IsoLatin9Char::try_from(char).is_ok(), "{char}");
        }

        for char in '\u{80}'..='\u{9F}' {
            assert_eq!(
                IsoLatin9Char::try_from(char),
                Err(IsoLatin9CharError::Invalid),
                "{char}"
            );
        }

        todo!()
    }
}

/// A ISO8859-1 encoded, growable string.
///
/// # Examples
/// TODO
///
/// # ISO8859-1
/// TODO
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct IsoLatin9String {
    bytes: Vec<u8>,
}

impl IsoLatin9String {
    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub const fn new() -> Self {
        todo!()
    }

    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub fn from_iso8859_1(vec: Vec<u8>) -> Result<Self, FromIso8859_1Error> {
        todo!()
    }

    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub fn into_bytes(self) -> Vec<u8> {
        todo!()
    }

    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub const fn capacity(&self) -> usize {
        todo!()
    }

    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub fn reserve(&mut self, additional: usize) {
        todo!()
    }

    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub fn reserve_exact(&mut self, additional: usize) {
        todo!()
    }

    // You guys got the idea. Try to replicate the String API into the type here.
}

impl fmt::Debug for IsoLatin9String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TIP: Usually for string types the debug implementation is the same as the display
        // implementation but with double quote before and after the text.
        todo!()
    }
}

impl fmt::Display for IsoLatin9String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Docs: TODO
/// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
#[derive(Debug)]
pub struct FromIso8859_1Error {
    // TODO
}

#[cfg(test)]
mod string_tests {
    use super::*;

    #[test]
    fn new() {
        let s = IsoLatin9String::new();
        assert_eq!(s.capacity(), 0);
    }

    #[test]
    fn with_capacity() {
        let s = IsoLatin9String::with_capacity(10);
        assert_eq!(s.capacity(), 10);
    }

    #[test]
    fn from_iso8859_1() {
        // Good case
        let s = IsoLatin9String::from_iso8859_1(vec![0x41, 0x42, 0x43]).unwrap();
        assert_eq!(s.capacity(), 3);
        assert_eq!(s.bytes, vec![0x41, 0x42, 0x43]);

        // Bad case
        // Contains invalid characters
        let res = IsoLatin9String::from_iso8859_1(vec![0x41, 0x42, 0x87, 0x44]);
        assert!(res.is_err()); // FIXME: Ideally, we should have a more specific error type checking here.
    }

    #[test]
    fn into_bytes() {
        let s = IsoLatin9String::from_iso8859_1(vec![0x41, 0x42, 0x43]).unwrap();
        assert_eq!(s.into_bytes(), vec![0x41, 0x42, 0x43]);
    }

    #[test]
    fn capacity() {
        let s = IsoLatin9String::from_iso8859_1(vec![0x41, 0x42, 0x43]).unwrap();
        assert_eq!(s.capacity(), 3);
    }

    #[test]
    fn reserve() {
        let mut s = IsoLatin9String::from_iso8859_1(vec![0x41, 0x42, 0x43]).unwrap();
        s.reserve(10);
        assert!(s.capacity() >= 13);
    }

    #[test]
    fn reserve_exact() {
        let mut s = IsoLatin9String::from_iso8859_1(vec![0x41, 0x42, 0x43]).unwrap();
        s.reserve_exact(10);
        assert_eq!(s.capacity(), 13);
    }
}
