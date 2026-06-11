## 22. Build System & Tooling

Build scripts (`build.elem`) are evaluated at compile time. `@lazy` elements serve as
thunks representing build targets, evaluated only when invoked:

```rs
import std.compiler as _;

let @pub @lazy build_frontend = {
    let opts := default_opts
        |> set_output(.wasm, "dist/frontend.wasm")
        |> add_lib("web")
        |> set_opt_level(3);

    async compile(opts);
    wait
};

let @pub @lazy build_backend = {
    let opts := default_opts
        |> set_output(.native, "dist/server")
        |> add_lib("http")
        |> add_lib("database");

    async compile(opts);
    wait
};

let @pub @lazy build_all = {
    build_frontend;
    build_backend
};
```

**`sh-mode`**: when the compiler flag `sh-mode` is active, unresolved identifiers are
treated as shell commands, blending OS-level scripting with Element's functional pipelines.
Sandboxing model is pending specification.

---

