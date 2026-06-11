# Plugins

Plugins are activated compilation surfaces, not just passive namespaces.

## Current Model

- `import` loads a pure module namespace.
- `load` activates a plugin surface.
- plugin exports may include runtime elements, compile-time elements, macros,
  initializers, and lifecycle behavior.

## Declaration Shape

A file declares itself a plugin with `plugin Name` at the top.

```rust
package myproject
plugin first
```

Loaded plugins may define:

- exported runtime elements
- exported `@comptime` elements
- syntax macros
- initializers
- package transforms
- plugin lifecycle behavior

## Loading

```rust
load std::compiler
load myproject::auth_plugin
```

`load` includes the plugin's exported runtime and compile-time surface. It does not
require a second `import` for the same export surface.

Loading a plugin must not auto-run arbitrary package-mutation code. Package and module
mutation belongs in explicit transforms. Runtime or build lifecycle behavior belongs in
`ini` declarations that are selected by the CLI or invoked with `call`.

## Package-Level Transforms

Plugin files may attach package transforms above the package declaration.

```rust
@package_transform(ffi_bindgen)
package myproject
plugin first
```

Libraries may export transform functions for users to attach manually.

```rust
load routes::plugin

@package_transform(routes::install(.dir = "src/routes"))
package app
plugin first
```

An explicitly called `ini` may configure build values such as `CompilerWorkspace`, but it
must not mutate an already parsed package AST. Package mutation must happen through the
transform pipeline.

## Open Areas

- Whether pure modules may export syntax macros without becoming plugins.
- How plugin initialization failures should be recovered.
- Whether `std::compiler` is a module, plugin, or split surface.

Related open questions live in [[open-questions]].
