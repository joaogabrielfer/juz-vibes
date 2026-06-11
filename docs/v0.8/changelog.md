# Changelog

## 2026-06-05 - v0.8.1

### Added

- Added additional open design records for the effects discussion: the `Fallible` and
  repair boundary, stream metadata and convergence limits, IO capability and mocking
  direction, async actor/channel comparisons, and future type-system feature candidates.
- Added a new reader-oriented wiki structure in `index.md` with separate `Start Here`,
  `Language Reference`, `Standard Library`, `Toolchain`, and `Design Notes` areas.
- Added draft high-level reference pages for the reorganized language chapters.
- Added a new draft standard-library area with `std.prelude`, `std.core.types`,
  `std.core.traits`, `std.error`, `std.effects`, `std.compiler`, and `std.pvm`.
- Added migrated standalone pages that replace the older numbered chapter layout.
- Added draft standard-library pages for collections, formatting, IO, math, text,
  filesystem, and networking.
- Added explicit default prelude notation declarations for arithmetic, comparison,
  logical, append, shift, named prefix, and handler-oriented notation surfaces.
- Added `markdown-conventions.md` to document why `txt` fences remain for grammar, CLI
  shapes, diagnostics, and plain output.

### Changed

- Updated the v0.8 revision marker to `0.8.1`.
- Changed the wiki navigation from flat numbered ordering to task-oriented navigation.
- Changed `std.functional` documentation into the new standard-library structure with a
  declaration-style module surface.
- Changed the open-questions map to track the default prelude boundary explicitly.
- Changed the new language-reference pages from thin draft maps into standalone migrated
  chapters with examples and implementation-facing rules.
- Clarified that Element has no intrinsic value-level operators; notation shape is core
  syntax, while default operators are standard-library/prelude declarations.
- Changed `std.prelude` so `print` is the lowest-level output surface, `echo` calls
  `print` after `Display.show`, and `panic` prints its message then exits with status `1`.

### Removed

- Removed the legacy numbered chapter files after migrating their content into the new
  wiki structure.

## 2026-06-03 - v0.8.0

### Added

- Created `v0.8` as a new minor documentation version copied from `v0.7`.
- Added `Effects & Handlers` as a new section.
- Added `def ... :: effect`, `@effects(...)`, `@handles(...)`, effect operation notation,
  and the initial core effects model.
- Added `Yield<T>`, `stream`, and `collect<C>` as the replacement for old
  list-comprehension yield syntax.
- Added `IO` effect rules for lazy uncached effectful zero-argument elements, mocking,
  compile-time control, and memoization restrictions.
- Added `@recoverable(...)`, `Repair<Error.X>`, `repair e`, `recover`, and one-shot
  `resume` for repairable errors.
- Added an inventory of language-defined attributes.
- Added async and actor effect direction with `Await<T>`, `Spawn<M>`, `Send<M>`, and
  `Receive<M>` reserved as future shapes.

### Changed

- Changed list-comprehension guidance to retire the old syntax in favor of `collect` and
  `stream`.
- Changed concurrency notes to point coroutine streams at the `Yield<T>` effect and to
  frame future actor operations as mailbox-specific effects.
- Changed thunk semantics so effectful zero-argument elements are lazy but not implicitly
  cached.

### Open Questions

- Multi-shot resumptions, general block-handler syntax, async scheduling, cancellation,
  actor supervision, typestate, and refinement types remain open.

## 2026-06-02 - v0.7.0

### Added

- Created `v0.7` as a new minor documentation version copied from `v0.6`.
- Added `expandable sum` examples centered on `Error` as an open error family.
- Added `sum ... with { ... }` attached-subelement model for sum roots.
- Added package-scope validation rules for `@requires(...)` trait requirements on member
  families.
- Added reflection notes for expandable sums and attached sum subelements.

### Changed

- Changed idiomatic error carrier from `Result<T, E>` examples to `Fallible<T>`.
- Changed `?` customization point from `Fallible` trait semantics to `IntoFallible<T>`.
- Changed local error-capture examples to explicit binder form `?(name)`.
- Changed kind-system examples to use `Fallible : Type -> Type` and `Either` for two-arg
  constructor partial application.
- Changed initializer examples from `Result<(), Error>` to `Fallible<()>`.
- Changed open-questions references from `Result` display behavior to `Fallible` display
  behavior.

## 2026-05-30 - v0.6.0

### Added

- Created v0.6 as a new minor documentation version copied from v0.5.
- Added explicit `package` declarations and package-transform placement rules.
- Added scoped transform attributes: `@transform`, `@module_transform(...)`, and
  `@package_transform(...)`.
- Added the meta-only type model for `TokenStream`, `Code`, `Item`, `Module`, `Package`,
  `SubElementInfo`, `AttributeInfo`, `Span`, `Diagnostic`, and `Patch<T>`.
- Added a dedicated `Attributes & Metadata` section for declared attributes, typed
  payloads, and untyped `@meta`.

### Changed

- Changed the metaprogramming chapter to distinguish syntax macros, bang-callable
  `@comptime` elements, and Phase-3 transforms.
- Changed syntax macro examples to use parenthesized bang invocation by default; brace-only
  forms now require explicit call-shape attributes.
- Changed `@embed(...)` examples to plugin- or library-provided `embed!` comptime calls.
- Changed the general member terminology from `field` to `subelement` where member
  semantics are not storage-only.
- Changed plugin loading rules so `load` includes the plugin's exported runtime and
  compile-time surface.

## 2026-05-30 - v0.5.0

### Added

- Added the unified `@notation` declaration model for symbolic and named notations.
- Added `@adjacent(left/right)` to control no-whitespace notation adjacency without token
  splitting.
- Added explicit tokenization rule that notation declarations never split identifier
  tokens.
- Added `std.functional` as the import-only module for functional notations and helpers.

### Changed

- Renamed `Operator System` to `Notation System`.
- Changed notation configuration attributes (`@position`, `@bind`, `@assoc`,
  `@precedence`, `@adjacent`) to declaration-level placement above notation `let`
  declarations.
- Changed `|~` and `$~` placeholders from `_` to `~`.
- Changed `$`, `$$`, and `$$$` semantics to mirror positional pipe insertion behavior.
- Changed `|>?` to be part of the core pipe precedence level and restricted to
  `Fallible`-implementing types.
- Changed `echo`, `panic`, `todo`, and `cast` wording from intrinsic parser special forms
  to named prefix notation declarations.

## 2026-05-30 - v0.4.1

### Added

- Added a dedicated `Kinds & Higher-Kinded Types` section with `Kind` grammar, explicit
  and sugar kind syntax, partial type-constructor application, and compile-time kind
  introspection.
- Added type-level pipeline examples for nested type construction in type-expression
  contexts.
- Added higher-order trait documentation (`FunctorK`) and complex nested mapping examples.

### Changed

- Clarified that `Type` reflection and `Kind` classification are separate subsystems.
- Clarified `<$>` as infix functor mapping desugaring to `Functor.map`.

### Open Questions

- Added rollout tracking for higher-order kinds and advanced trait-resolution ergonomics.

## 2026-05-30 - v0.4.0

### Added

- Created v0.4 as a new minor documentation version copied from v0.3.
- Added module, plugin, package, circular import, and visibility rules.
- Added `ini` declarations, `call`, reserved CLI initializer names, default recipe shape,
  `pyx` CLI structure, and basic `pvm.run(...)` usage.
- Added built-in string interpolation macro behavior.
- Added union type syntax with `|` in type positions.
- Added `~` unnamed lambda arguments.
- Added `Fallible` as the customization point for `?`.
- Added `open-questions.md` with grouped unresolved design threads.

### Changed

- Changed `def` syntax to always use `::`.
- Changed `::` to mean standalone type declarations only.
- Changed traits into one unified `def Name :: trait { ... }` system.
- Changed implementation syntax to `@impl(...)`, `@auto_impl(...)`, and `extends`.
- Changed spread from `...` to `..`.
- Changed semicolon rules to apply only inside multi-statement blocks and after
  `return`/`be`.
- Changed `be` into a readability alias for `return`.
- Changed `println` to accept exactly one `string`; `echo` remains generic over `Display`.
- Changed override syntax to declaration-level `@override(priority: N)`.

### Removed

- Removed the copied `whole-docs.md` aggregate from v0.4.
- Removed the retired `type` keyword.
- Removed the old JAI-style `let MAX :: 100` constant syntax.
- Removed `impl for` syntax.
- Removed the syntax-level distinction between behavioral traits and type traits.
- Removed `be` as an inner block-yield mechanism.

### Open Questions

- `std.compiler` API shape, async build scripts, CLI-exposed `ini` rules, entry point API,
  custom macro syntax, `sh-mode` sandboxing, `pvm` API, `be` scope, initializer arguments
  and return values, `watch`, and carried v0.3 unresolved areas remain open.
