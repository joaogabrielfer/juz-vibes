# Juz / Pyx

This repository is the early implementation workspace for Juz, a programming language
whose current design docs live under `site/docs/`.

The integrated toolchain CLI is `pyx`. The implementation is intentionally small right
now: it is focused on establishing the repository shape, bytecode output, and the first
compiler path before broader language features are added.

Documentation site: <https://joaogabrielfer.github.io/juz-vibes/>

## Current Status

This is work in progress.

Today, `pyx build <file.juz>` supports only a minimal source shape:

```juz
ini main := {
    42
}
```

That source is compiled into a `.pbc` bytecode file using the layout documented in
`bytecode-spec.md`.

The current compiler does not yet implement the full language described in
`site/docs/v0.12`.
Unsupported syntax should be treated as expected until the relevant feature is planned
and implemented.

## Workspace Layout

```txt
crates/
  pyx-bytecode/   .pbc bytecode data structures and encoder
  pyx-compiler/   minimal Juz-to-bytecode compiler
  pyx-cli/        pyx command-line interface
site/             Astro/Starlight documentation site and docs source
tests/fixtures/   source fixtures for tests and manual checks
```

Planned future workspace areas include:

```txt
crates/
  pyx-vm/         pvm implementation, once brought into this monorepo
  pyx-pasm/       PAsm support
  pyx-lsp/        language server
editors/          editor integrations
```

## Tooling Direction

The intended tool split is:

- `pyx`: integrated CLI for build, run, test, watch, execution, and orchestration
- `pyxc`: compiler frontend for Juz and PAsm
- `pyxi`: interpreter frontend for Juz, with possible future WASM embedding
- `pvm`: virtual machine for `.pbc` bytecode

These may eventually be separate binaries or links/wrappers around shared toolchain
libraries. For now, only the `pyx` binary exists.

## Commands

Build a Juz file into bytecode:

```sh
cargo run --bin pyx -- build tests/fixtures/minimal-main.juz
```

Choose an explicit output path:

```sh
cargo run --bin pyx -- build tests/fixtures/minimal-main.juz -o build/minimal-main.pbc
```

Run checks:

```sh
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Build the documentation site:

```sh
cd site
npm ci
npm run build
```

## Documentation

Language feature work should follow the latest docs version under `site/docs/`. At the
moment, that is `site/docs/v0.12`.

The docs are design documentation, not a complete implementation status report. When
implementing a feature, use the docs as the intended language behavior and add tests for
the implemented subset.

The documentation site lives in `site/` and renders the latest version from `site/docs/`.
The site source is checked in; generated Starlight content, build output, and
dependencies are not.

Useful documentation entry points:

- Source index: [`site/docs/v0.12/index.md`](site/docs/v0.12/index.md)
- Toolchain docs: [`site/docs/v0.12/toolchain-overview.md`](site/docs/v0.12/toolchain-overview.md)
- CLI/build docs: [`site/docs/v0.12/build-system-and-cli.md`](site/docs/v0.12/build-system-and-cli.md)
- Bytecode file format: [`bytecode-spec.md`](bytecode-spec.md)

## CI

The CI is split by project area:

- Rust checks run when Rust workspace, fixtures, CI, or bytecode spec files change.
- Site checks run only when files under `site/` or the site workflow change.

## Bytecode And VM Boundary

`.pbc` files currently follow `bytecode-spec.md`.

The VM implementation is expected to be brought into this repository later. Until then,
the compiler should treat the bytecode spec and any provided `pvm` binary as the
compatibility boundary.

## Repository Direction

This repository is expected to become a monorepo for the language implementation,
compiler, VM, tooling, and documentation surfaces. The current workspace layout is meant
to keep those pieces separate while allowing them to share compiler and bytecode
libraries.
