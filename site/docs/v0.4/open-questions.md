# Open Questions

This file collects unresolved design issues for v0.4. Items marked as carried from v0.3
were already open before the post-v0.3 update.

## Topic Map

| Topic | Questions worth a focused thread |
|---|---|
| Metaprogramming | Custom macro syntax, macro export/import rules, Phase 2, `@auto_impl` internals |
| Modules and tooling | `std.compiler` API, CLI-exposed `ini` rules, entry point API, `watch`, package behavior |
| Initializers and runtime | `ini` arguments/returns, plugin failure recovery, `pvm` runtime API |
| Async and concurrency | Build-script async, general async/await, coroutines, actor mailbox typing |
| Syntax cleanup | `be` scope, list-comprehension yield syntax, `=` vs `:=` style |
| Type system and runtime | Numeric tower, memory model, lifetimes, error hierarchy, custom operator resolution |
| Standard library | Full standard library surface, formatting/debug traits, I/O, networking, collections |
| Kind system rollout | Staging higher-order kinds and advanced trait-resolution ergonomics |

## Higher-Order Kind Rollout

Status: Open

Context:
v0.4 now specifies `Kind`, kinded traits, and `FunctorK`-style higher-order traits. The
remaining question is rollout depth for compiler inference and diagnostics in early
implementations.

Options:
1. Ship unary constructor kinds (`Type -> Type`) first, then stage higher-order kinds.
2. Ship full `(Type -> Type) -> Type` support immediately.
3. Parse full syntax immediately but gate some inference paths behind feature flags.

Current leaning:
Stage support: stabilize unary kind inference and error quality first, then broaden to
higher-order trait-heavy patterns.

## `std.compiler` API Shape

Status: Open

Context:
The build script imports `std.compiler` as a module, but the compiler subsystem may need
plugin-level lifecycle hooks.

Options:
1. Keep `std.compiler` as a normal module that exposes pure build configuration helpers.
2. Make compiler integration a plugin with `ini` declarations and lifecycle hooks.
3. Split the API: pure module helpers plus an optional compiler plugin.

Current leaning:
The examples use `import std.compiler`, so the module API is the current working shape.

## Async in Build Scripts

Status: Open

Context:
Build examples use `await $ compile(...)` to express parallel compilation. The general
async model is unspecified.

Options:
1. Define a build-only async subset for parallel compilation tasks.
2. Design the full async/await model before finalizing build scripts.
3. Treat build concurrency as compiler scheduling rather than language-level async.

Current leaning:
A build-only subset may be enough initially, but it should not block the later general
async model.

## CLI-Flagged Initializers

Status: Open

Context:
`ini` declarations can be selected from the CLI, but exposure rules are not final.

Options:
1. All top-level `ini` declarations in `first.jz` are CLI-accessible.
2. An attribute such as `@cli` marks specific `ini` declarations.
3. The entry-point `ini` is excluded from CLI access unless explicitly exposed.

Current leaning:
No final rule.

## Entry Point Customization

Status: Open

Context:
`main` is the default entry point but can be renamed in the build recipe. The target must
be an `ini`.

Options:
1. `set_entrypoint("my_ini")`
2. `set_entrypoint(my_ini)`
3. A structured entrypoint descriptor in `std.compiler`

Current leaning:
The examples use `set_entrypoint(...)`, but the exact API remains TBD.

## Custom Macro Syntax

Status: Open

Context:
The language may support macros that parse non-Element grammars such as SQL or HTML.

Example:

```rs
db |> fetch!(SELECT * FROM users WHERE id = ${user_id})
```

Required capabilities:

1. Declare trigger syntax and delimiters
2. Parse arbitrary syntax up to a terminator
3. Report domain-specific errors
4. Integrate type safety, such as SQL return schemas

Current leaning:
This belongs in a dedicated metaprogramming chapter.

## `sh-mode` Sandboxing

Status: Open

Context:
`set_flag(sh_mode)` enables shell command passthrough for unresolved identifiers.

Options:
1. No sandbox; trust explicit `sh_mode`.
2. Capability-based permissions.
3. Project-level allowlist.
4. CLI prompt or policy file for dangerous commands.

Current leaning:
No final security model.

## `pvm` Runtime API

Status: Open

Context:
Build scripts import `std.pvm` and call `pvm.run(...)`, but the full runtime API is not
specified.

Known surface:

```rs
pvm.run("build/app.pbc")
```

Current leaning:
Keep the documented surface minimal until runtime embedding and process semantics are
designed.

## `be` Scope

Status: Open

Context:
`be` is confirmed as a semantic alias for `return`, not a block-yield mechanism. The
style boundary is not formal.

Questions:

1. Can `be` appear in a `[]` arm body?
2. Can `be` appear in a `use` continuation block?
3. Should `be` be accepted anywhere `return` is accepted?
4. Should diagnostics recommend `be` in constant-like bindings?

Current leaning:
Semantically, `be` should be accepted wherever `return` is accepted unless a later style
rule narrows it.

## List-Comprehension Yield Syntax

Status: Open

Context:
v0.3 used `be` to yield elements from list comprehensions. v0.4 changes `be` to an alias
for `return`, so the old comprehension examples are invalid.

Options:
1. Introduce a distinct `yield` keyword.
2. Use bare final expressions in comprehension bodies.
3. Use a builder/combinator model instead of special comprehension yield syntax.

Current leaning:
No final replacement.

## Initializer Argument and Return Semantics

Status: Open

Context:
Initializers can be written with typed arguments and return types.

```rs
ini deploy := (target: string) -> Result<(), Error> { ... }
```

Questions:

1. How are CLI arguments parsed and typed?
2. How do return values surface to the CLI?
3. Does a non-`()` return print, affect exit code, or feed another `ini`?
4. How are `Result` errors displayed?

Current leaning:
No final rule.

## `watch` Default Implementation

Status: Open

Context:
`watch` has a suggested default behavior: watch files and rerun.

Options:
1. Compiler built-in behavior.
2. Shipped as `std.compiler.plugins.default_recipe`.
3. Template-provided `ini watch`.

Current leaning:
No final rule.

## Binding Operator Normalization

Status: Open

Context:
The post-v0.3 update confirms `let name = value` as a value-only inferred form, while many
current examples still use `let name := value`.

Options:
1. Keep both forms and treat `:=` as preferred inferred shorthand.
2. Retire `:=` for `let` and use `=` for all value definitions.
3. Keep `:=` for fresh inferred declarations and reserve bare `=` for definitions that
   follow a `::` signature or have an inline `:` annotation.

Current leaning:
v0.4 documents both forms and tracks the style choice here.

## Macro Export and Import Rules

Status: Open

Context:
Macro code may need to run when imported modules are compiled, but imports should remain
free of general side effects.

Questions:

1. How does a module declare exported macros?
2. Does importing a macro execute only its expansion code or also setup code?
3. Can plugins export macros to modules that only use `import`?

Current leaning:
Modules should explicitly declare exported macros, and the compiler should execute only
those at import time.

## Plugin Initialization Recovery

Status: Deferred

Context:
The current rule is that a failing plugin `ini` crashes the program. Recovery is not
available yet.

Current leaning:
Keep crash-on-failure until the broader error-handling and supervision models mature.

## Carried from v0.3

Status: Open unless noted

| Area | Notes |
|---|---|
| Memory model | Ownership, allocation strategy, garbage collection vs arenas |
| Full numeric primitive tower | Overflow behavior, casting rules, literal typing |
| Coroutines / yield-as-stream | Interaction with actors and the retired `be` yield role |
| Actor mailbox typing | Typed channels and supervision trees |
| Error type hierarchy | Rust-like ergonomics without `Box<dyn Error>` |
| Full standard library surface | Collections, I/O, networking, math, formatting |
| Metaprogramming Phase 2 | The gap between token macros and comptime execution |
| Lifetime / borrow semantics | Whether needed given the VM model |
| Variadic functions | Relationship between `..T`, value groups, and arrays |
| Operator resolution for custom types | How operators resolve on unknown or generic types |
