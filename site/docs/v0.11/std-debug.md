# std::debug

This page documents debug-oriented output and formatting surfaces.

Debug utilities are always available through the standard surface. A release build must
not make debug declarations disappear from name resolution or type checking. Build policy
may choose whether debug calls print, become no-ops, are routed to a debug sink, or are
reported as warnings for a specific target profile.

## Draft Surface

```rust
module std::debug

def Debug :: trait {
    show_debug: Self -> string,
}

let debug: <T: Debug> T -> void
let debug_with: <T: Debug> T, DebugOptions -> void
```

## Formatting Hook

String interpolation with `:?` uses the debug formatting hook.

```rust
println("Debug: {packet:?}")
```

The default lowering target is `Debug::show_debug(value)`. Types may implement both
`Display` and `Debug`; `Display` is for user-facing text and `Debug` is for diagnostic
inspection.

## Release Behavior

Release behavior is controlled by the build profile or target policy. The language rule
is that debug declarations remain valid in all build modes.
