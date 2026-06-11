# Runtime pvm

`pvm` executes SLUR bytecode produced by the compiler.

## Current Documented Surface

- compiled artifacts use the `.pbc` extension
- build scripts may invoke `pvm::run(...)`
- the embedding and host-control story is still open

## Separation

- the existence of `pvm` as a runtime is part of the architecture
- the callable module surface is documented in [[std-pvm]]
- the CLI-facing launch flow is documented in [[build-system-and-cli]]
