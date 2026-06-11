## 1. Core Philosophy & Architecture

Element is built on a single unifying principle: **Everything is an Element**. Variables,
functions, types, traits, and modules are structurally identical constructs. A variable is a
zero-argument function. A function is a named lambda. There is no categorical distinction
between data and behavior.

Three foundational rules govern all Element code:

- **Immutable by default** — all bindings are immutable unless explicitly marked `mut`
- **Private by default** — all elements are module-local unless marked `@pub`
- **Expression-oriented** — every block and construct evaluates to a value; the final
  expression in any block is its implicit return value

**Compilation target:** SLUR, a custom flat-stack virtual machine. The stack-based
architecture enables zero-cost abstractions for value groups, pattern matching, and error
propagation without heap allocation overhead. Because the VM operates on a flat data stack,
tuple destructuring, currying, and monadic chaining unroll directly into sequential
bytecode with no boxing.

---

