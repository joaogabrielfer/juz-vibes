## 23. Initializers, Build System & CLI

### `ini`

`ini` declares lifecycle-bound side-effectful functions. Unlike `let`, an `ini`
declaration runs only when explicitly invoked with `call` or selected by the CLI.

```rs
ini setup_db := {
    DB.connect(ENV.DATABASE_URL)
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

Custom initializers can have typed arguments and return types:

```rs
ini deploy := (target: string) -> Result<(), Error> {
    deploy_to(target)
}
```

Initializer argument passing and return value surfacing are still open.

### Calling Initializers

Initializers are invoked with `call`, not regular function call syntax.

```rs
call setup_db
call deploy "production"
```

Regular calls such as `setup_db()` are not used for initializers.

### Reserved CLI Initializers

`build`, `run`, `test`, and `watch` are reserved `ini` names with special CLI semantics.
In `first.jz`, defining those names as plain `let` bindings is a compile error; the
compiler should suggest `ini` instead.

```rs
ini build := { ... }
ini run := {
    call build;
    pvm.run("build/app.pbc")
}
```

`main` is the default entry point but is not special at the language level. The build
recipe can rename the entry point. The selected entry point must be an `ini`.

### Build Script Structure

```rs
plugin first

import std.compiler { set_flag, set_entrypoint, set_output, add_lib, compile }
import std.pvm

let @const BUILD_DIR := "./build/"

ini build := {
    let embed_wasm := compiler.default_opts
        |> set_output(.pbc(BUILD_DIR <> "script.pbc"))
        |> set_flag(sh_mode)
        |> set_entrypoint(None);

    let backend := compiler.default_opts
        |> set_output(.pbc(BUILD_DIR <> "backend.pbc"));

    let frontend := compiler.default_opts
        |> set_output(.wasm(BUILD_DIR <> "frontend.wasm"))
        |> set_entrypoint(None)
        |> add_lib("web");

    await $ compile(embed_wasm);
    await $ compile(frontend);
    await $ compile(backend)
}

ini run := {
    call build;
    pvm.run("build/script.pbc");
    pvm.run("build/backend.pbc")
}
```

The `await $ compile(...)` syntax is a placeholder until the async model is specified.
For the build system, async is needed to express parallel compilation tasks.

### Default Build Recipe

Simple projects can load a default recipe.

```rs
load std.compiler.plugins.default_recipe

ini main := {
    // program logic
}
```

Example CLI behavior:

```txt
pyx foo.jz run
```

The default recipe creates `build/foo.pbc` and runs it.

### CLI Structure

```txt
pyx <file> [ini_name]       # run ini in specific file
pyx [ini_name]              # search for ini in first.jz in current directory
pyx init <name>             # scaffold new project
pyx init <name> <template>  # scaffold with custom template
```

`pyx init` creates `first.jz` with the default build script and `src/<name>.jz`.
Templates live in `$HOME/.juz/init/` and can be user-defined.

```txt
pyx init myapp
pyx init myserver webserver
pyx init myscript juzscript
```

### `watch`

`watch` has a suggested default behavior: watch for file changes and re-run. Whether this
is compiler built-in behavior or provided by `std.compiler.plugins.default_recipe` is
open. Users may override it:

```rs
ini watch := {
    // user override
}
```

### Plugin Initialization Failure

If an `ini` in a loaded plugin fails, the program crashes. No recovery mechanism exists at
this stage. This may be revisited when the error-handling story matures.

### `pvm`

Build scripts may import `std.pvm` to run compiled bytecode:

```rs
import std.pvm

pvm.run("build/app.pbc")
```

The full `pvm` runtime API surface is still unspecified.
