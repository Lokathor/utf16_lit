# Changelog

# 2.0.0

* Thanks to wonderful work by [Plecra](https://github.com/Plecra) the crate is now a macro_rules instead of a proc-macro.
* **Breaking:** The output is now a `[u16; N]` array (of whatever size) rather than a `&[u16]`.
  If you still want to use `&[u16]` just put a `&` before the macro use.
* **Minor Breaking:** The Minimum Rust Version is now 1.46 instead of 1.45

# 1.0.0

* Initial release.
