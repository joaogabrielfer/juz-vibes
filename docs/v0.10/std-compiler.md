# std.compiler

This page starts documenting the compiler-facing standard module and plugin helpers used
by build scripts.

## Draft Surface

```rust
module std.compiler

let default_opts   :: CompilerOptions
let set_flag       :: CompilerOptions, CompilerFlag   -> CompilerOptions
let set_entrypoint :: CompilerOptions, Option<string> -> CompilerOptions
let set_output     :: CompilerOptions, OutputTarget   -> CompilerOptions
let add_lib        :: CompilerOptions, string         -> CompilerOptions
let add_package    :: string                          -> (BuildRecipe -> BuildRecipe)
let include_module :: string, string, string          -> (BuildRecipe -> BuildRecipe)
let compile        :: CompilerOptions                 -> Task<Fallible<void>>
```

## Notes

- This surface is still highly provisional.
- The docs currently describe `std.compiler` as a module, but plugin integration and
  build-recipe ownership remain open.
- This page exists to begin extracting toolchain-facing APIs from the flat language spec.

## Related Pages

- [[build-system-and-cli]]
- [[compiler-pipeline]]
- [[open-questions]]
