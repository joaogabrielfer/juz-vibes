# Standard Library

This area documents user-level surfaces that are meant to be implemented, imported,
extended, or replaced outside the compiler core.

Element does not reserve value-level operators as intrinsic language operations. The
language reference specifies notation syntax and evaluation rules; the standard library
specifies the default declarations that make common notation usable.

## Organization

- [[std-prelude]] documents the default always-available surface.
- [[std-core-types]] documents canonical data carriers.
- [[std-core-traits]] documents canonical traits and abstraction contracts.
- [[std-functional]] documents import-only functional helpers and notations.
- [[std-error]] documents the standard error carrier and expandable error family.
- [[std-effects]] documents standard effects and handler declarations.
- [[std-compiler]] documents compiler-facing build helpers.
- [[std-pvm]] documents VM runtime helpers.

## Core Boundary

The compiler owns:

- tokenization
- notation placement and precedence
- evaluation order
- effect visibility checking
- type checking
- overload/coherence resolution

The standard library owns:

- default arithmetic notation declarations
- default comparison and logic declarations
- `Display` and formatting hooks
- `Fallible<T>`, `Error`, and `IntoFallible<T>`
- `Yield<T>`, `IO`, `stream`, `collect`, `recover`, and `resume`
- import-only functional notation such as `>>=`, `<$>`, and `<*>`

## Declaration-Style Documentation

Standard-library pages use declaration-style surfaces, similar to a header file.

```rust
module std.example

def Example<T> :: prod {
    value: T,
}

let make: <T> T -> Example<T>
```

These declarations define the intended public shape. Implementation bodies are included
only when they clarify semantics.

## Open Areas

The following modules are expected but not yet specified in detail:

- `std.collections`
- `std.io`
- `std.math`
- `std.text`
- `std.format`
- `std.debug`
- `std.net`
- `std.fs`

Until those pages exist, references to collection types such as `Arr<T>`, `Set<T>`,
`Stream<T>`, and `Task<T>` remain provisional standard-library names.
