## 1. Core Philosophy & Architecture

Element is built on a single unifying principle: **Everything is an Element**. Variables,
functions, types, traits, modules, plugins, and initializers are language-level elements.
A variable is a zero-argument function. A function is a named lambda. There is no hard
runtime category boundary between data and behavior.

Three foundational rules govern all Element code:

- **Immutable by default** - all bindings are immutable unless explicitly marked `mut`
- **Private by default** - all elements are module-local unless marked with a public
  visibility attribute
- **Expression-oriented** - blocks evaluate to values; the final expression in a block is
  the implicit return value

### Language Identity

The language name is pending. The current design may rename **Element** to **Juz** and use
the `.jz` source extension. Until that decision is finalized, this specification continues
to describe the language as Element.

The toolchain names are confirmed regardless of the final language name:

| Tooling name | Meaning |
|---|---|
| `pyx` | Command-line tool |
| `pvm` | Pyx Virtual Machine runtime |
| `.pbc` | Compiled bytecode extension |

Name alternatives under consideration:

| Name | Extension | Notes |
|---|---|---|
| `Juz` | `.jz` | Current preference; short and energetic |
| `Pryx` | `.px` | Matches `pyx` toolchain naming |
| `Flux` | `.fx` | Conveys the flow-oriented philosophy |
| `Vex` | `.vx` | Short and distinct |
| `Nex` | `.nx` | From "nexus"; everything connects |
| `Lace` | `.lc` | Suggests weaving primitives together |
| `Kine` | `.kn` | From "kinetic"; motion and flow |

### Compilation Target

Element compiles to SLUR, a custom flat-stack virtual machine executed by `pvm`. The
stack-based architecture enables zero-cost abstractions for value groups, pattern matching,
and error propagation without heap allocation overhead. Because the VM operates on a flat
data stack, tuple destructuring, currying, and monadic chaining can unroll directly into
sequential bytecode with no boxing.
