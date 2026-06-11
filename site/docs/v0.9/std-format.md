# std.format

This page documents formatting surfaces used by `echo`, `println`, and string
interpolation.

## Draft Surface

```rust
module std.format

def FormatSpec :: prod {
    raw: string,
}

def Format :: trait {
    show_fmt: (Self, FormatSpec) -> string,
}

let format: <T: Display> (T) -> string
let format_with: <T: Format> (T, FormatSpec) -> string
let println: (string) -> ()
```

## Interpolation Hooks

Default lowering hooks:

- `{expr}` -> `Display.show(expr)`
- `{expr:?}` -> debug formatting hook
- `{expr:spec}` -> formatting hook with string specifier payload

```rust
println("User: {user}")
println("Value: {n:.2f}")
println("Debug: {packet:?}")
```

## Boundary

String interpolation syntax is compiler-provided and documented in [[metaprogramming]].
The formatting hooks and traits are standard-library surface.

`print` is documented in [[std-prelude]] as the current lowest-level output surface.
`println` is a formatting convenience layered above it.
