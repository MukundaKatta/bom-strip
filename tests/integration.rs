use bom_strip::{detect_bom, strip_all, strip_bytes, strip_str, Bom};

#[test]
fn utf8_bom() {
    let b = [0xEF, 0xBB, 0xBF, b'h', b'i'];
    assert_eq!(detect_bom(&b), Some(Bom::Utf8));
    assert_eq!(strip_bytes(&b), &[b'h', b'i']);
}

#[test]
fn utf16_le() {
    let b = [0xFF, 0xFE, b'a', 0];
    assert_eq!(detect_bom(&b), Some(Bom::Utf16Le));
}

#[test]
fn utf16_be() {
    let b = [0xFE, 0xFF, 0, b'a'];
    assert_eq!(detect_bom(&b), Some(Bom::Utf16Be));
}

#[test]
fn utf32_le_not_misread_as_utf16_le() {
    let b = [0xFF, 0xFE, 0, 0, b'a', 0, 0, 0];
    assert_eq!(detect_bom(&b), Some(Bom::Utf32Le));
    assert_eq!(strip_bytes(&b), &[b'a', 0, 0, 0]);
}

#[test]
fn utf32_be() {
    let b = [0, 0, 0xFE, 0xFF, 0, 0, 0, b'a'];
    assert_eq!(detect_bom(&b), Some(Bom::Utf32Be));
}

#[test]
fn no_bom() {
    let b = b"hello";
    assert_eq!(detect_bom(b), None);
    assert_eq!(strip_bytes(b), b"hello");
}

#[test]
fn strip_str_leading_only() {
    assert_eq!(strip_str("\u{FEFF}hello"), "hello");
    assert_eq!(strip_str("mid\u{FEFF}word"), "mid\u{FEFF}word");
}

#[test]
fn strip_all_removes_internal() {
    assert_eq!(strip_all("a\u{FEFF}b\u{FEFF}c"), "abc");
}

#[test]
fn bom_len_table() {
    assert_eq!(Bom::Utf8.len(), 3);
    assert_eq!(Bom::Utf16Le.len(), 2);
    assert_eq!(Bom::Utf32Le.len(), 4);
}
