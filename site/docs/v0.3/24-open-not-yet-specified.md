## 24. Open / Not Yet Specified

The following areas have been identified but not yet formally specified. They will be
addressed in future chapters of this document.

| Area                                 | Status          | Notes                                                 |
| ------------------------------------ | --------------- | ----------------------------------------------------- |
| Memory model                         | 🔲 Not started  | Ownership, allocation strategy, GC vs arena           |
| Module system                        | 🔲 Not started  | Declaration, imports, namespacing, visibility scoping |
| Full numeric primitive tower         | 🔲 Not started  | Overflow behaviour, casting rules, literals           |
| Async / await                        | 🔲 Not started  | General model vs build-system-only                    |
| Coroutines / yield-as-stream         | 🔲 Concept only | Interaction with actor model                          |
| Actor mailbox typing                 | 🔲 Not started  | Typed channels, supervision trees                     |
| `sh-mode` sandboxing                 | 🔲 Concept only | Security model for shell integration                  |
| Error type hierarchy                 | 🔲 Deferred     | Rust-like but more ergonomic; no `Box<dyn Error>`     |
| String interpolation                 | 🔲 Not started  | `format("Hello {}", name)` or `"Hello {name}"`        |
| Full standard library surface        | 🔲 Not started  | Collections, I/O, networking, math                    |
| Import / module syntax               | 🔲 Not started  | `import`, namespacing, re-exports                     |
| Metaprogramming Phase 2              | 🔲 Not started  | The gap between token macros and comptime             |
| Lifetime / borrow semantics          | 🔲 Not started  | If applicable given target VM model                   |
| Variadic functions                   | 🔲 Not started  | `Arr<T>` splat vs true variadic                       |
| Operator resolution for custom types | 🔲 Not started  | How `+` on unknown types is resolved at compile time  |
