# Design Principles

Element is organized around one unifying rule: everything is an element. Values,
functions, traits, types, modules, plugins, and initializers live in one language model.

Three design constraints shape the rest of the spec:

- Immutable by default: bindings are immutable unless marked `mut`.
- Private by default: declarations are module-local unless published explicitly.
- Expression-oriented: blocks evaluate to values and the final expression is the result.

The language is also intentionally biased toward:

- Zero-cost value grouping and destructuring on the SLUR stack machine.
- Uniform declaration forms with `let` for value-level elements and `def` for type-level
  elements.
- A strong separation between parser rules and library-provided notations.

Compilation target:

- Source code compiles to SLUR bytecode.
- `pvm` executes `.pbc` output.
- The VM model explains why tuples, destructuring, and pattern dispatch are documented as
  direct language concepts instead of wrapper-library features.

Related chapters:

- [[evaluation-model]]
- [[toolchain-overview]]
