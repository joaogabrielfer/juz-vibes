# Changelog

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
