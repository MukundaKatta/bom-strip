//! # bom-strip
//!
//! Strip UTF-8/16/32 BOMs and stray U+FEFF code points from text.
//!
//! A leading byte order mark breaks `serde_json::from_str`, hash-based
//! deduplication, and config parsers that don't allow leading
//! whitespace. This crate gives you four small functions:
//!
//! - [`strip_str`] — strip a leading U+FEFF from a `&str`.
//! - [`strip_all`] — strip every U+FEFF in the input, not just leading.
//! - [`strip_bytes`] — strip a leading UTF-8 / UTF-16 LE/BE / UTF-32
//!   LE/BE BOM from a `&[u8]`.
//! - [`detect_bom`] — identify which BOM (if any) leads `&[u8]`.
//!
//! ## Example
//!
//! ```
//! use bom_strip::{strip_str, strip_bytes, detect_bom, Bom};
//!
//! assert_eq!(strip_str("\u{FEFF}hello"), "hello");
//! assert_eq!(strip_bytes(&[0xEF, 0xBB, 0xBF, b'h', b'i']), &[b'h', b'i']);
//! assert_eq!(detect_bom(&[0xFF, 0xFE, b'a', 0]), Some(Bom::Utf16Le));
//! ```

#![deny(missing_docs)]

/// Identified BOM kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bom {
    /// `EF BB BF` (UTF-8)
    Utf8,
    /// `FE FF` (UTF-16 big-endian)
    Utf16Be,
    /// `FF FE` (UTF-16 little-endian)
    Utf16Le,
    /// `00 00 FE FF` (UTF-32 big-endian)
    Utf32Be,
    /// `FF FE 00 00` (UTF-32 little-endian; check before UTF-16 LE)
    Utf32Le,
}

impl Bom {
    /// Length of this BOM in bytes.
    ///
    /// A BOM always occupies at least two bytes, so there is no
    /// corresponding `is_empty` method.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(self) -> usize {
        match self {
            Bom::Utf8 => 3,
            Bom::Utf16Be | Bom::Utf16Le => 2,
            Bom::Utf32Be | Bom::Utf32Le => 4,
        }
    }
}

/// Detect which BOM (if any) leads `b`.
pub fn detect_bom(b: &[u8]) -> Option<Bom> {
    if b.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Some(Bom::Utf8);
    }
    // 4-byte BOMs must be checked before 2-byte to avoid misidentifying
    // a UTF-32 LE BOM (`FF FE 00 00`) as a UTF-16 LE BOM (`FF FE`).
    if b.starts_with(&[0xFF, 0xFE, 0x00, 0x00]) {
        return Some(Bom::Utf32Le);
    }
    if b.starts_with(&[0x00, 0x00, 0xFE, 0xFF]) {
        return Some(Bom::Utf32Be);
    }
    if b.starts_with(&[0xFE, 0xFF]) {
        return Some(Bom::Utf16Be);
    }
    if b.starts_with(&[0xFF, 0xFE]) {
        return Some(Bom::Utf16Le);
    }
    None
}

/// Strip a leading BOM from `b`. Returns the input unchanged if none.
pub fn strip_bytes(b: &[u8]) -> &[u8] {
    match detect_bom(b) {
        Some(bom) => &b[bom.len()..],
        None => b,
    }
}

/// Strip a leading U+FEFF from `s`.
pub fn strip_str(s: &str) -> &str {
    s.strip_prefix('\u{FEFF}').unwrap_or(s)
}

/// Strip every U+FEFF (BOM and zero-width no-break-space) in `s`.
pub fn strip_all(s: &str) -> String {
    s.chars().filter(|c| *c != '\u{FEFF}').collect()
}
