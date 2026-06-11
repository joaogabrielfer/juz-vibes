# Build System and CLI

This page owns `ini`, `call`, reserved initializer names, build recipe shape, and the
user-facing `pyx` CLI.

## `ini`

`ini` declares lifecycle-bound side-effectful functions. Unlike `let`, an `ini`
declaration runs only when explicitly invoked with `call` or selected by the CLI.

```rust
ini setup_db := {
    DB::connect(ENV::DATABASE_URL)
}

ini main := {
    call setup_db;
    start_server()
}
```

The syntax is:

```txt
ini name := { body }
```

Custom initializers can have typed arguments and return types.

```rust
ini deploy: (target: string) -> Fallible<unit> = {
    deploy_to(target)
}
```

Initializer argument passing and return value surfacing are still open.

## Calling Initializers

Initializers are invoked with `call`, not regular function call syntax.

```rust
call setup_db
call deploy "production"
```

Regular calls such as `setup_db()` are not used for initializers.

## Reserved CLI Initializers

`build`, `run`, `test`, and `watch` are reserved `ini` names with special CLI semantics.
In `first.jz`, defining those names as plain `let` bindings is a compile error.

```rust
ini build := { ... }
ini run := {
    call build;
    pvm::run("build/app.pbc")
}
```

`main` is the default entry point but is not special at the language level. The build
recipe can rename the entry point. The selected entry point must be an `ini`.

## Build Script Structure

```rust
package myproject
plugin first

import std::compiler {
    CompilerWorkspace,
    set_package_root,
    set_source,
    set_flag,
    set_entrypoint,
    set_output,
    add_lib,
    compile,
}
import std::pvm

let @const BUILD_DIR := "./build/"

ini build := {
    let base := CompilerWorkspace()
        |> set_package_root("./")

    let embed_wasm := base
        |> set_source("src/script.jz")
        |> set_output(.pbc(BUILD_DIR <> "script.pbc"))
        |> set_flag(sh_mode)
        |> set_entrypoint(None)

    let backend := base
        |> set_source("src/backend.jz")
        |> set_output(.pbc(BUILD_DIR <> "backend.pbc"));

    let frontend := base
        |> set_source("src/frontend.jz")
        |> set_output(.wasm(BUILD_DIR <> "frontend.wasm"))
        |> set_entrypoint(None)
        |> add_lib("web")

    await $ compile(embed_wasm);
    await $ compile(frontend);
    await $ compile(backend);
}

ini run := {
    call build;
    pvm::run("build/script.pbc");
    pvm::run("build/backend.pbc");
}
```

Each `CompilerWorkspace` value describes one compilation target. A build script that
needs several outputs should construct several workspaces and compile them separately.
The `await $ compile(...)` syntax is a placeholder until the async model is specified.

## Tests

`test "name" { ... }` declares a unit test. Test declarations are excluded from normal
builds and included by `pyx test` or by an explicitly selected `ini test`.

```rust
test "add_one" {
    assert! add_one(2) == 3
}
```

The default `pyx test` behavior is:

1. load the package through the normal build recipe
2. include test declarations
3. call `ini test` when the package defines it
4. otherwise run the default test harness

`ini test` may customize setup, filtering, reporting, or runtime capabilities, but it
does not change the meaning of `test` declarations.

## Default Build Recipe

Simple projects can load a default recipe.

```rust
package myproject
plugin first

load std::compiler::plugins::default_recipe

ini main := {
    // program logic
}
```

Example CLI behavior:

```txt
pyx foo.jz run
```

The default recipe creates `build/foo.pbc` and runs it.

## CLI Structure

CLI shapes are plain terminal syntax, so they use `txt` fences.

```txt
pyx <file> [ini_name]
pyx [ini_name]
pyx init <name>
pyx init <name> <template>
```

`pyx init` creates `first.jz` with the default build script and `src/<name>.jz`.
Templates live in `$HOME/.juz/init/` and can be user-defined.

```txt
pyx init myapp
pyx init myserver webserver
pyx init myscript juzscript
```

## `watch`

`watch` has a suggested default behavior: watch for file changes and re-run. Whether this
is compiler built-in behavior or provided by `std::compiler::plugins::default_recipe` is
open.

```rust
ini watch := {
    // user override
}
```

## Plugin Initialization Failure

If an `ini` in a loaded plugin fails, the program crashes. No recovery mechanism exists at
this stage. This may be revisited when the error-handling story matures.

## Boundary

- `ini` syntax and `call` are language-level.
- compiler helper APIs belong in [[std-compiler]].
- runtime execution helpers belong in [[std-pvm]].
