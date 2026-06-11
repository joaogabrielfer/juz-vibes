# std::compiler

This page starts documenting the compiler-facing standard module and plugin helpers used
by build scripts.

## Draft Surface

```rust
module std::compiler

def CompilerWorkspace :: prod {
    package_root: Path,
    source: Path,
    entrypoint: Option<string>,
    output: OutputTarget,
    flags: Set<CompilerFlag>,
    libs: Arr<string>,
}

let CompilerWorkspace :: void -> CompilerWorkspace

let set_package_root :: CompilerWorkspace, Path           -> CompilerWorkspace
let set_source       :: CompilerWorkspace, Path           -> CompilerWorkspace
let set_flag         :: CompilerWorkspace, CompilerFlag   -> CompilerWorkspace
let set_entrypoint   :: CompilerWorkspace, Option<string> -> CompilerWorkspace
let set_output       :: CompilerWorkspace, OutputTarget   -> CompilerWorkspace
let add_lib        :: CompilerWorkspace, string         -> CompilerWorkspace
let add_package    :: string                            -> (BuildRecipe -> BuildRecipe)
let include_module :: string, string, string            -> (BuildRecipe -> BuildRecipe)
let compile        :: CompilerWorkspace                 -> Task<Fallible<void>>
```

## Notes

- This surface is still highly provisional.
- The docs currently describe `std::compiler` as a module, but plugin integration and
  build-recipe ownership remain open.
- `CompilerWorkspace` describes one compilation target. It is not a container for
  multiple outputs.
- Build scripts may construct multiple independent `CompilerWorkspace` values when they
  need to compile several targets.
- A future orchestration type such as `BuildPlan` may group several workspaces, but that
  coordination should stay separate from the per-target workspace.

## Related Pages

- [[build-system-and-cli]]
- [[compiler-pipeline]]
- [[open-questions]]
