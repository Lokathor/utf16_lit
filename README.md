[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
![min-rust](https://img.shields.io/badge/Min%20Rust-1.45-green.svg)
[![crates.io](https://img.shields.io/crates/v/utf16_lit.svg)](https://crates.io/crates/utf16_lit)
[![docs.rs](https://docs.rs/utf16_lit/badge.svg)](https://docs.rs/utf16_lit/)

# utf16_lit

Lets you change a standard Rust string literal (utf8) into a utf16 value at compile time.

This crate does **not** use `syn` or `quote`, so it actually builds fast.
