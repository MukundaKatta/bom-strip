# bom-strip

[![crates.io](https://img.shields.io/crates/v/bom-strip.svg)](https://crates.io/crates/bom-strip)

Strip UTF-8/16/32 BOMs and stray U+FEFF code points from text before
parsing or hashing.

```rust
use bom_strip::{strip_str, strip_bytes, detect_bom, Bom};
assert_eq!(strip_str("\u{FEFF}hello"), "hello");
assert_eq!(strip_bytes(&[0xEF, 0xBB, 0xBF, b'h', b'i']), &[b'h', b'i']);
assert_eq!(detect_bom(&[0xFF, 0xFE, b'a', 0]), Some(Bom::Utf16Le));
```

Handles UTF-32 LE before UTF-16 LE (same first two bytes). Zero deps.
MIT or Apache-2.0.
