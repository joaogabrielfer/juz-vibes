# Open Questions

This file collects unresolved design issues for v0.6. Items marked as carried from v0.3
were already open before later documentation updates.

## Topic Map

| Topic | Questions worth a focused thread |
|---|---|
| Metaprogramming | Custom macro syntax, transform ordering, Phase 2, `@auto_impl` internals |
| Modules and tooling | `std.compiler` API, CLI-exposed `ini` rules, entry point API, `watch`, package behavior |
| Attributes and metadata | `@meta` scope, attribute-schema ergonomics, meta reflection API final surface |
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
db |> fetch!({ SELECT * FROM users WHERE id = ${user_id} })
```

Required capabilities:

1. Declare trigger syntax and delimiters
2. Parse arbitrary syntax up to a terminator
3. Report domain-specific errors
4. Integrate type safety, such as SQL return schemas

Current leaning:
This belongs in a dedicated metaprogramming chapter.

## Compile-Time Call-Shape Grammar

Status: Open

Context:
v0.6 allows `@macro` and bang-callable `@comptime` declarations to reuse call-shape
attributes such as `@position(...)` and `@bind(...)`. The released docs establish the
default parenthesized form and the fact that brace-only calls require explicit
call-shape attributes, but the full capture grammar is not final.

Questions:

1. What are the complete bindable capture sources for compile-time calls: `left`, `right`,
   `block`, delimiter-specific groups, or others?
2. How should multi-argument prefix forms such as `parse_cli_commands! args { ... }` be
   declared precisely?
3. Should brace-only calls always lower to one `block` binding, or may they carry
   multiple captures?
4. How do precedence and adjacency interact with bang-callable `@comptime` declarations
   in complex expressions?

Current leaning:
Keep the default invocation model simple and parenthesized, and require explicit
call-shape attributes for every non-default macro or comptime bang-call form.

## Transform Ordering and Fixed-Point Rules

Status: Open

Context:
v0.6 introduces explicit item, module, and package transforms. The scope ladder is
documented, but transform scheduling details remain open.

Questions:

1. What is the canonical ordering between item, module, and package transforms?
2. How many fixed-point iterations are allowed before the compiler aborts?
3. Do emitted items restart item-level transforms in the same pass or only in the next
   iteration?
4. How should diagnostics explain transform cycles or non-converging rewrites?

Current leaning:
Deterministic scope order with an explicit iteration cap and cycle diagnostics.

## String Interpolation Customization

Status: Open

Context:
v0.6 specifies string interpolation as a built-in compiler-provided literal transform
that lowers forms such as `"{foo} - {bar}"` into concatenation and formatting hooks.
The surface syntax is intentionally compact, but the customization model is not final.

Questions:

1. What are the canonical formatting hooks for `{expr}`, `{expr:?}`, and `{expr:spec}`?
2. Should the language standardize traits such as `Display`, `Debug`, and `FormatSpec`,
   or use another hook model?
3. How much of interpolation should be intrinsic compiler behavior versus library-defined
   formatting handlers?
4. Should interpolation support custom specifier namespaces beyond the core format/debug
   forms?

Current leaning:
Keep interpolation syntax built-in and lightweight, and expose customization through
formatting traits or compiler-owned hook points rather than through user-defined
token-stream macros.

## Meta Reflection API Final Surface

Status: Open

Context:
v0.6 introduces meta-only types such as `Item`, `Module`, `Package`, `SubElementInfo`,
`AttributeInfo`, and `Patch<T>`, but many listed properties are intentionally documented
as likely rather than final.

Questions:

1. Which properties on `Item`, `Module`, `Package`, and `SubElementInfo` are guaranteed
   by the stable meta API?
2. Should transforms mutate through replacement-only `Patch<T>` values, more granular
   edit operations, or both?
3. How much unresolved compiler state, if any, should meta APIs expose?
4. Should typed code fragments remain one untyped `Code` surface initially, or should
   typed code handles such as expression or item fragments be added early?

Current leaning:
Keep the initial meta API narrow, stable, and explicitly scoped; add more granular edit
surfaces only when real transform use cases demand them.

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

## Package Declaration Inference

Status: Open

Context:
v0.6 adds explicit `package` declarations and requires them in package entry files such
as `first.jz`. The remaining open issue is how much inference should exist in simpler
single-file or light-build workflows.

Questions:

1. In simple mode, may the compiler infer `package` from the project root or file path?
2. Should package inference be permitted only outside package-scoped transforms?
3. If a file omits `package`, should that be a warning, an error, or a separate
   compatibility mode?
4. How should inferred package names interact with build recipes and default templates?

Current leaning:
Require explicit package names in canonical package entry files and allow inference only
as a convenience mode if it does not weaken package-scoped metaprogramming.

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

## Pipe and Monad-Bind Mixing

Status: Open

Context:
v0.5 places `|>?` inside the core pipe family and moves '>>=' to `std.functional`.
Both can express chained fallible/monadic flows, but they intentionally belong to
different notation families.

Questions:

1. Should expressions that mix `|>?` and '>>=' require parentheses unconditionally?
2. If mixed chains are allowed without parentheses, what precedence and associativity
   should '>>=' use relative to the pipe family?
3. Should diagnostics suggest an idiomatic rewrite when `|>?` can replace '>>=' in a
   fallible pipeline?

Current leaning:
Require explicit parentheses when mixing `|>?` with '>>=' to keep parsing predictable and
to preserve the style boundary between fallible pipelines and general monadic bind flows.

## Module-Exported Macro Rules

Status: Open

Context:
Plugins now activate through `load`, which includes runtime and compile-time exports.
What remains open is whether pure modules imported through `import` may export syntax
macros or other compile-time helpers without becoming plugins.

Questions:

1. How does a module declare exported macros?
2. Does importing a macro execute only its expansion code or also setup code?
3. Should pure modules be allowed to export syntax macros at all, or should that surface
   be plugin-only?

Current leaning:
If modules export macros, the export surface should be explicit and imports should remain
side-effect free apart from expansion work required for compilation.

## Privileged Compile-Time Plugin Capabilities

Status: Open

Context:
This thread explored plugin-backed compile-time facilities such as `bash!` and `python!`.
The docs now distinguish `load` from `import`, but they do not yet specify the policy
model for privileged compile-time plugins that touch the host shell, interpreters, or
other external tools.

Questions:

1. Which capabilities require plugins rather than pure modules?
2. Should host-integrating macros such as `bash!` and `python!` be part of the standard
   distribution, optional plugins, or third-party packages?
3. How should permission, reproducibility, and diagnostics behave for host-integrating
   compile-time plugins?
4. Should privileged plugins declare explicit capabilities in metadata or activation
   syntax?

Current leaning:
Keep host-integrating compile-time features plugin-backed and capability-oriented rather
than modeling them as ordinary pure modules.

## `@meta` Scope and Status

Status: Open

Context:
v0.6 documents `@meta` as a possible untyped metadata escape hatch, but this thread did
not finalize whether it should ship as part of the core language, remain tooling-only, or
be deferred.

Questions:

1. Should `@meta` be part of the core language surface in early versions?
2. If present, should `@meta` accept arbitrary dotted keys, arbitrary values, or a more
   constrained schema?
3. How should `@meta` interact with declared typed attributes on the same target?
4. Should tooling be allowed to consume `@meta` without compiler awareness of its keys?

Current leaning:
Prefer declared typed attributes for stable semantics and keep `@meta` as a secondary,
explicitly looser escape hatch if it ships at all.

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
