# std::math

This page is a placeholder for the standard math surface.

Arithmetic notation declarations are sketched in [[std-prelude]]. Numeric capability
traits are documented in [[std-core-traits]].

Current numeric baseline:

- `int` is a stable alias for `i64`
- `float` is a stable alias for `f64`
- `usize` and `isize` are pointer-sized
- integer literals begin as exact compile-time integers
- float literals begin as exact compile-time float literals
- `Byte` is a standard alias for `u8`

Overflow behavior, checked conversion APIs, and the broader concrete math API remain open
in this revision.
