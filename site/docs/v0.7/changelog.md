# Changelog

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
