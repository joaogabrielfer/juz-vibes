# Overview

This wiki is being reorganized around reader tasks instead of feature arrival order.

The current structure has four top-level zones:

- `Start Here` introduces the language and the state of the spec.
- `Language Reference` documents parser-visible syntax and core semantics.
- `Standard Library` documents user-level elements, notations, and module surfaces.
- `Toolchain` documents package loading, plugins, build scripts, the CLI, and the VM.

The older numbered chapters have been migrated into the new structure and removed from
this version folder.

The intended boundary is:

- Core language: syntax, typing rules, evaluation rules, and compiler-owned phases.
- Standard library: names, traits, effects, helper notations, and modules that user code
  can define, import, implement, or replace.
- Toolchain: the build system, compiler plugin model, package integration, and runtime.

See also:

- [[design-principles]]
- [[language-reference]]
- [[standard-library]]
- [[toolchain-overview]]
