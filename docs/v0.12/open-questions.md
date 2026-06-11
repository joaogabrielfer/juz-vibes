# Open Questions

This file collects unresolved design issues for v0.12. Items marked as carried from v0.3
were already open before later documentation updates.

## Topic Map

| Topic | Questions worth a focused thread |
|---|---|
| Metaprogramming | Custom macro syntax, transform ordering, Phase 2, `@auto_impl` internals |
| Modules and tooling | CLI-exposed `ini` rules, entry point API, `watch`, package behavior, multi-target build orchestration |
| Initializers and runtime | `ini` arguments/returns, plugin failure recovery, `pvm` runtime API |
| Effects and handlers | Fallible/repair boundary, multi-shot resumptions, handler imports, stream metadata, cancellation cleanup |
| Async and concurrency | Build-script async, general async/await, actor mailbox typing, Go-style channels vs actor mailboxes |
| Syntax cleanup | `be` scope, `=` vs `:=` style |
| Pattern matching | View/extractor groups, pattern synonyms, map patterns, regex/literal macro details |
| Type system and runtime | Numeric overflow, cast APIs, memory model, lifetimes, typestate, refinement proof depth, custom operator resolution |
| Standard library | Full standard library surface, prelude boundary, I/O, networking, collections, filesystem path ergonomics |
| Kind system rollout | Staging higher-order kinds and advanced trait-resolution ergonomics |
| Future type-system features | Session types, linear/quantitative types, algebraic subtyping |

## Prelude Boundary

Status: Open

Context:
The wiki now separates the standard library from the language reference, but the exact
always-available surface is still unsettled.

The current draft treats default operators as prelude declarations rather than intrinsic
language operations. This makes the language model uniform, but it exposes design
pressure around short-circuiting, numeric literal typing, and whether some names should be
available in every module.

Questions:

1. Which standard traits belong in the default prelude?
2. Should `yield`, `stream`, `collect`, `recover`, and `resume` all be always available?
3. Should `print`, `echo`, `panic`, `todo`, and `cast` stay prelude-level defaults or move to
   explicit imports?
4. How should diagnostics explain a missing prelude item when the user disables or
   shadows the default surface?

Current leaning:
Keep the prelude explicit and separated from import-only notation modules such as
`std::functional`. `std` is addressable by default through explicit `std::...` paths.
`print` is the current lowest-level output surface; `panic` prints its message through
that surface and exits with status `1`.

## Short-Circuiting Standard Notations

Status: Open

Context:
The current draft documents `&&` and `||` as standard prelude notations. If all notations
are ordinary eager functions, these cannot short-circuit.

Options:
1. Treat `&&` and `||` as syntax recognized by the compiler while still resolved through
   prelude declarations.
2. Add lazy-argument or thunked-operand support to notation declarations.
3. Define `&&` and `||` as ordinary eager functions and accept no short-circuiting.

Current leaning:
Do not silently make them eager. Either the notation system needs lazy operands, or the
compiler needs a precise lowering rule for these specific prelude names.

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

## Multi-Target Build Orchestration

Status: Open

Context:
`CompilerWorkspace` now describes one compilation target. A build script can create
multiple independent workspaces and compile them separately.

Questions:

1. Should a later `BuildPlan` type coordinate several `CompilerWorkspace` values?
2. Should parallel compilation be expressed by general async syntax, build-script
   scheduling, or a compiler helper?
3. How should shared cache, dependency graph, and diagnostics be represented across
   several workspaces?

Current leaning:
Keep `CompilerWorkspace` single-target. Add orchestration only as a separate layer if
real build scripts need it.

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
3. A structured entrypoint descriptor in `std::compiler`

Current leaning:
The examples use `set_entrypoint(...)`, but the exact API remains TBD.

## Custom Macro Syntax

Status: Open

Context:
The language may support macros that parse non-Element grammars such as SQL or HTML.

Example:

```rust
db |> fetch!({ SELECT * FROM users WHERE id = ${user_id} })
```

Required capabilities:

1. Declare trigger syntax and delimiters
2. Parse arbitrary syntax up to a terminator
3. Report domain-specific errors
4. Integrate type safety, such as SQL return schemas

Current leaning:
This belongs in a dedicated metaprogramming chapter.

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
Build scripts import `std::pvm` and call `pvm::run(...)`, but the full runtime API is not
specified.

Known surface:

```rust
pvm::run("build/app.pbc")
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
2. Can `be` appear in a `with` continuation block?
3. Should `be` be accepted anywhere `return` is accepted?
4. Should diagnostics recommend `be` in constant-like bindings?

Current leaning:
Semantically, `be` should be accepted wherever `return` is accepted unless a later style
rule narrows it.

## Retired List-Comprehension Syntax

Status: Resolved in v0.8

Context:
v0.3 used `be` to yield elements from list comprehensions. v0.4 changed `be` to an alias
for `return`, so the old comprehension examples became invalid.

Resolution:
v0.8 retires the old list-comprehension syntax. Eager comprehensions use `collect` and
lazy comprehensions use `stream`, both backed by the `Yield<T>` effect.

Current leaning:
Use `collect<Arr> { ... }`, `collect<Set> { ... }`, or `stream { ... }`.

## Effects and Handler Semantics

Status: Open

Context:
v0.8 introduces effect declarations, `@effects`, `@handles`, `Yield<T>`, `IO`,
recoverable repair, `stream`, and `collect`. The initial model intentionally avoids
multi-shot resumptions and full async semantics.

Questions:

1. Should non-core effect operation notation require explicit named imports or importing
   the whole effect declaration?
2. Should handlers other than `stream`, `collect`, and `recover` be user-definable with
   block operands in v0.8-era syntax?
3. Should one-shot resumption be enforced syntactically, through runtime validation, or
   both?
4. How should diagnostics explain an unhandled visible effect on a public declaration?

Current leaning:
Keep the first surface small: explicit imports for user-defined effect notation, one-shot
resumptions, and dedicated handler forms before general block-handler syntax.

## Fallible and Repair Boundary

Status: Open

Context:
The current error model keeps `Fallible<T>` as plain inert data. A discussed alternative
was treating `Fallible` as a "maybe effect carrier", where an `Error` value could carry a
resumable continuation. That approach was rejected as the main model because it would make
ordinary `.Err` values non-plain: they could become one-shot, resource-owning,
non-serializable, and unsafe to store or log casually.

The current direction is:
- `Fallible<T>` remains ordinary data,
- `@recoverable(R)` attaches a repair response protocol to an `Error` member,
- resumability exists only while a callee explicitly performs `repair e`,
- if no handler responds, `repair e` returns `.None` and the callee may return `.Err(e)`.

Questions:

1. Should the compiler forbid storing recoverable `Error` values in long-lived structures,
   or is that unnecessary because the value itself carries no continuation?
2. Should `Repair<Error::X>` be a visible effect whenever `repair e` appears, or should it
   be inferred as part of `Fallible`-returning function analysis?
3. Should `recover` arms be allowed to inspect and transform unrelated non-recoverable
   errors, or should they match recoverable variants only?
4. Should diagnostics explicitly say "this `.Err` cannot be resumed; only `repair e` can
   resume" when users try to recover after a plain failure has already returned?

Current leaning:
Keep resumability out of `Fallible` values. `Fallible` should stay matchable, storable,
serializable, and loggable. Recovery is a dynamic repair opportunity exposed by `repair e`,
not a hidden continuation stored inside `Error`.

## Generator and Stream Metadata

Status: Open

Context:
`stream` and `collect` now replace old list comprehensions through `Yield<T>`. Earlier
brainstorming also considered lazy evaluated lists and compile-time inspection such as
`list.converges`.

Full convergence detection is not decidable for arbitrary user code. The compiler or
macro system cannot generally prove whether an arbitrary stream is finite or infinite.

Possible metadata shape:

```rust
def StreamShape :: sum {
    Finite(Option<int>),
    Infinite,
    Unknown,
}
```

Known producers could expose conservative shape information:

```rust
range(0, 10).shape   // Finite(.Some(10))
naturals().shape     // Infinite
filter(source, pred) // Unknown or bounded by source
```

Questions:

1. Should `Stream<T>` expose a standard `.shape` or `.bounds` property?
2. Should `collect` reject statically known infinite streams, warn, or allow them?
3. Should macros and comptime code be allowed to query stream metadata without evaluating
   the stream?
4. Should the language later distinguish `Stream<T>` from `Generator<Yielded, Returned>`?

Current leaning:
Expose only conservative metadata. `Stream<T>` should be the first user-facing abstraction.
`Generator<Y, R>` may be added later if final generator return values become important.

## IO Capabilities and Mocking

Status: Open

Context:
`IO` is useful because it affects caching, compile-time safety, mocking, sandboxing, and
optimization. The current spec treats `IO` as one effect, but testing and build security
may need a more precise capability model.

Possible future directions:

```rust
IO
IO<FileSystem>
IO<Network>
IO<Clock>
IO<Process>
```

Questions:

1. Should `IO` remain one broad effect, or split into capability-specific effects?
2. Should tests provide mock `IO` through effect handlers, runtime capability objects, or
   both?
3. Should compile-time code be able to request specific IO capabilities from the build
   policy?
4. Should effectful zero-argument declarations always be uncached, or can an explicit
   handler define cache semantics for a specific capability?

Current leaning:
Begin with one broad `IO` effect. Add capability-specific IO only when sandboxing,
mocking, or build policies need that precision.

## Filesystem Path Ergonomics

Status: Open

Context:
`std::fs` is still a placeholder. A future `Path` type should support ergonomic path
construction without making strings pretend to be paths.

Possible direction:

```rust
let config_path: Path = fs::home_dir_path / ".config"
```

Questions:

1. Should `/` be a `Path`-only join notation?
2. Should string literals auto-convert to path segments in this context?
3. How should absolute paths, normalization, platform separators, and invalid segments be
   handled?

Current leaning:
Keep this as future `std::fs` design. The notation is promising, but it should be tied
to a concrete `Path` API and must not make ordinary string division meaningful.

## Async, Actors, and Channels

Status: Open

Context:
Effects appear suitable for async because a blocked computation can perform a suspension
operation and wait for a handler or scheduler to resume it. The current docs reserve
`Await<T>`, `Spawn<M>`, `Send<M>`, and `Receive<M>` as future shapes.

Comparison notes:
- Go uses goroutines plus channels. Channels are shared communication objects; sends and
  receives may block depending on buffering.
- Elixir uses isolated lightweight processes with mailboxes, message passing, linking,
  monitoring, tasks, and supervision.
- Element's existing sketch leans closer to isolated actors with typed mailboxes than to
  shared Go-style channels.

Questions:

1. Should Element expose Go-like channel values, actor mailboxes, or both?
2. Should `receive` be an effect operation, a pattern-matching form, or both?
3. Should `await` be syntax for receiving a task reply message, or a separate scheduler
   effect?
4. Should `spawn` return a typed `Pid<M>`, a task handle, or a richer process descriptor?
5. How do cancellation and cleanup interact with `with`, streams, and suspended async
   computations?
6. Should actor supervision be a library/runtime layer first, or part of the core effect
   model?
7. Should structured concurrency provide `parallel! { ... }` and `race! { ... }` syntax,
   or should those be ordinary handler/macros over `Task` and cancellation effects?

Current leaning:
Model async around effectful suspension and actor-style typed mailboxes. Keep supervision,
mailbox protocols, cancellation, and full scheduler behavior out of the first effects
revision.

Possible future syntax:

```rust
parallel! {
    fetch(foo);
    fetch(bar)
}

race! {
    do_thing();
    timeout(1000);
}
```

Do not finalize this syntax before `Task`, `Await`, cancellation, and cleanup semantics
are specified.

## Typestate and Refinement Proof Depth

Status: Open

Context:
v0.12 documents lightweight refinement types with local proof rules. The remaining
questions are how far those proofs should go and when the language should introduce
first-class typestate.

Current refinement surface:

```rust
def Port :: int where it >= 0 && it <= 65535
def NonEmpty<T> :: Arr<T> where it.len > 0
```

Possible typestate direction:

```rust
def File :: resource states {
    Closed,
    Open,
}
```

Questions:

1. Should `where` refinements stay limited to local compiler-known predicates, or should a
   later revision introduce stronger solver-backed reasoning?
2. Should typestate be a new declaration form or a pattern built from ordinary sum types
   and traits?
3. How should typestate interact with `with`, `IO`, and stream cleanup?

Current leaning:
Keep refinements lightweight and local for now. Typestate is a good future fit for
resources, transactions, actors, and protocols, but should wait until the effect and
`IO` model is stable.

## Safe Array Indexing and Dynamic Domain Semantics

Status: Open

Context:
v0.12 documents safe indexing for fixed arrays, dynamic arrays, and bounded dynamic
arrays with custom index domains.

Current documented surface:

```rust
let fixed :: int[3]
let lookup :: string[3: PersonId]
let values :: Arr<int>
let queue :: Arr<string>[PersonId]
```

The remaining unsettled point is the exact dynamic meaning of `Arr<T>[I]` and how long
flow-sensitive index proofs should remain valid in the presence of mutation.

Questions:

1. Should `Arr<T>[I]` be permanently defined as a dense prefix over a sequential domain,
   or should a future sparse keyed collection get a visually similar surface?
2. Should `arr.has_index(i)` refine only the index, or should it create an explicit
   temporary capability tied to that array value?
3. Which mutations invalidate an established proof: length change only, or also any
   operation that can reorder or remap indices?
4. Should `arr.indices` freeze the array for the loop body, or simply reject mutations
   that would invalidate the loop binder?
5. Should `get` remain `Option<T>` while `try_get` carries richer domain or occupancy
   diagnostics through `Fallible<T>`?

Current leaning:
Keep `Arr<T>[I]` as a dense bounded sequence over a sequential finite domain. Keep
proofs flow-sensitive and invalidate them on mutations that may change length,
occupancy, or index mapping. Reserve sparse keyed storage for a separate collection type.

## Pattern Matching Future Work

Status: Open

Context:
v0.12.2 formalizes the core pattern model: source-order matching, `|` alternatives,
`pattern as whole` aliases, pinned value patterns with `^name`, Rust-like product field
patterns, sequence rest patterns such as `[first, ..middle, last]`, full-match regex
patterns through `re!`, and conservative redundancy/exhaustiveness rules.

Several related features remain promising but are not part of the core model yet.

### View and Extractor Groups

Possible future syntax:

```rust
match url [
    do split(".") [
        [site, "com"] => println("{site} is a .com site"),
        [site, owner, "com"] => println("{site} is part of {owner}"),
        [..parts, "net"] => println("{} is a .net site", parts |> join(".")),
    ]

    _ => println("unknown site url"),
]
```

Possible semantics:
- `match` evaluates the outer scrutinee once.
- When dispatch reaches a `do view(args)` group, the scrutinee is inserted as the first
  argument, as in `scrutinee |> view(args)`.
- Nested arms match the transformed value in source order.
- If no nested arm matches, the `do` group fails and outer matching continues.
- View functions should be `@pure` at first. Effectful extractors would make pattern
  dispatch visibly procedural and would need explicit effect propagation.
- Ordinary `Option`, `Fallible`, and other sums should not be silently unwrapped. A
  partial extractor should expose its result explicitly unless a later extractor protocol
  defines failure semantics.

Questions:
1. Should `do view(args) [ ... ]` exist only as a match-arm group, or also as a nested
   pattern form?
2. Should a compact single-arm form such as `do split(".") [..parts, "net"] => ...` be
   accepted, or should the block form be the only surface?
3. Should extractor functions require an explicit marker such as `@extractor`, or is
   `@pure` enough?
4. Should partial extractors be limited to `Option<T>`, support named sums, or use a
   dedicated prism-like protocol?
5. Can a total view with exhaustive nested arms contribute to exhaustiveness checking, or
   should all `do` groups remain conservative runtime filters?

Current leaning:
Keep explicit pipe-before-match as the ordinary style when the whole match uses one
transformed representation:

```rust
match url |> split(".") [
    [site, "com"] => ...
    [..parts, "net"] => ...
]
```

If view groups are added later, prefer the block form. Treat it as pure, source-order,
branch-local precomposition of matching with a view function. Do not silently unwrap
`Option` or `Fallible`.

### Pattern Synonyms

Possible future syntax:

```rust
pattern DotCom(site: string): Arr<string> = [site, "com"]
```

Questions:
1. Should exported pattern synonyms require explicit input and binding types?
2. Do pattern synonyms expand before type checking, or do they participate as named
   pattern abstractions?
3. May a public pattern synonym expose private constructors or private product fields?
4. Can pattern synonyms contribute to exhaustiveness and redundancy checking?
5. Should generic pattern synonyms be allowed in the first version?

Current leaning:
Defer pattern synonyms until module privacy, product-pattern visibility, and exported
abstraction rules are more settled.

### Map and Dictionary Patterns

Possible future syntax:

```rust
match json [
    { "id": id, "name": name, ..rest } => ...
]
```

Questions:
1. Which concrete collection owns this syntax: hash maps, ordered maps, JSON-like
   objects, or any collection implementing a pattern protocol?
2. Which key equality and hashing rules are used?
3. Does `..rest` allocate a new map, borrow a view, or follow ownership of the scrutinee?
4. How should missing keys, duplicate keys, and key type inference be diagnosed?

Current leaning:
Defer map patterns until standard map types and key equality/hash traits are stable.

### Regex and Literal Macro Details

Context:
`re!` is currently specified as a literal macro that receives uninterpreted literal
contents and produces a compile-time checked `Regex`.

Questions:
1. Should `r!`, `b!`, and `c!` become the general raw-string, byte-string, and C-string
   literal macro family?
2. Should user-defined literal macros receive the same uninterpreted literal-token
   access as compiler-provided macros?
3. Should `re!` interpolation default to escaped literal insertion, raw regex-fragment
   insertion, or require explicit interpolation markers?
4. How should regex flags such as case-insensitive, multiline, dot-all, and Unicode modes
   be surfaced?
5. Should regex capture binders that might not participate bind `Option<string>` in a
   later revision instead of being rejected?

Current leaning:
Keep `re!` as the first concrete literal macro. Treat the broader literal macro family as
a metaprogramming design thread.

### Sequence Rest Representation

Context:
Sequence rest patterns bind remaining elements through `..name`.

Questions:
1. Does `..tail` over `Arr<T>` allocate a new `Arr<T>`, borrow a slice/view, or follow
   ownership of the original scrutinee?
2. Should sequence patterns be built into `Arr<T>` only at first, or generalized through
   a trait/protocol for sequence-like collections?
3. How should mutable arrays interact with rest views and invalidation of index proofs?

Current leaning:
Keep the surface as `Arr<T>`-oriented for now, but avoid promising eager copying. The
implementation should preserve recursive pattern ergonomics without making ordinary
head/tail recursion accidentally quadratic.

### GADT and Dependent-Like Pattern Narrowing

Possible future direction:

```rust
def Expr<T> :: sum {
    Int(int): Expr<int>,
    Bool(bool): Expr<bool>,
}

let eval: <T> Expr<T> -> T = [
    .Int(x) => x,
    .Bool(x) => x,
]
```

Questions:
1. Should sum constructors be allowed to refine the result type of a generic sum?
2. How should pattern matching introduce type equality constraints into arm bodies?
3. How would these constraints interact with union inputs, polymorphic `[]` functions,
   refinement types, and exhaustiveness checking?

Current leaning:
This is mathematically attractive but much larger than the current pattern model. Treat
it as a future type-system thread, not a near-term pattern feature.

## Future Type-System Feature Candidates

Status: Open

Context:
Several more academic language features were discussed as possible fits. They are not
part of the v0.8 effects update, but they should remain visible as future design threads.

Candidates:
- session types: statically describe message protocols for actors and mailboxes,
- linear or quantitative types: track values that must be used exactly once or in limited
  ways,
- algebraic subtyping: improve inference and checking for union-heavy and structural type
  systems,
- typestate: track resource and protocol state transitions through types,
- inferred generic constraints: infer constraints from unannotated function bodies such
  as `x, y -> x + y` after traits and notation constraints are stable.

Benefits:
- session types could make actor protocols safer,
- linear or quantitative types could prevent double-close, forgotten cleanup, and invalid
  resource duplication,
- algebraic subtyping could make union and pattern-matched function inference more
  powerful,
- typestate could improve files, transactions, actors, and `with`-managed resources.

Risks:
- session types can make actor APIs feel academic and rigid,
- linear or quantitative types change ordinary variable ergonomics,
- algebraic subtyping is a significant compiler implementation burden,
- typestate can become confusing when aliases or shared ownership are not settled.
- inferred generic constraints can produce confusing diagnostics if arithmetic traits,
  overflow behavior, cast APIs, and notation dispatch are not already stable.

Current leaning:
Typestate remains the most natural next candidate after the current refinement and
indexing work. Session types should wait for the actor model. Linear/quantitative types
and algebraic subtyping should be considered only if concrete implementation pressure
appears. Inferred generic constraints should stay on the backburner until overflow
behavior and cast APIs are stable.

## Initializer Argument and Return Semantics

Status: Open

Context:
Initializers can be written with typed arguments and return types.

```rust
ini deploy: (target: string) -> Fallible<unit> = { ... }
```

Questions:

1. How are CLI arguments parsed and typed?
2. How do return values surface to the CLI?
3. Does a non-`unit` return print, affect exit code, or feed another `ini`?
4. How are `Fallible` errors displayed in CLI output?

Current leaning:
No final rule.

## `watch` Default Implementation

Status: Open

Context:
`watch` has a suggested default behavior: watch files and rerun.

Options:
1. Compiler built-in behavior.
2. Shipped as `std::compiler::plugins.default_recipe`.
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

## Pipe and Monad-Bind Mixing

Status: Open

Context:
v0.5 places `|>?` inside the core pipe family and moves '>>=' to `std::functional`.
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
| Numeric conversion and overflow | Overflow behavior, casting rules, checked conversion APIs |
| Coroutines / yield-as-stream | Resolved into `Yield<T>`, `stream`, and `collect`; async interaction remains open |
| Actor mailbox typing | Typed channels and supervision trees |
| Full standard library surface | Collections, I/O, networking, math, formatting |
| Metaprogramming Phase 2 | The gap between token macros and comptime execution |
| Lifetime / borrow semantics | Whether needed given the VM model |
| Variadic functions | Relationship between `..T`, value groups, and arrays |
| Operator resolution for custom types | How operators resolve on unknown or generic types |
