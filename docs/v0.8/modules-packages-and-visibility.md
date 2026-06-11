# Modules, Packages, and Visibility

This chapter owns namespace and package structure.

## Modules

Modules are pure namespaces imported with `import`. Importing a module has no runtime
side effects.

```rust
import std.math
import std.collections { HashMap, Arr }
import myproject.utils as utils
```

One module is not required to be one file. A module is the set of files that share the
same module declaration. Multiple files in the same directory can belong to different
modules.

```txt
src/
  foo1.jz   // module foo
  foo2.jz   // module foo
  bar1.jz   // module bar
  bar2.jz   // module bar
```

Each file declares its module at the top.

```rust
module foo
```

Directory organization is idiomatic and recommended but not enforced.

Local modules use `<package_name>.<module_name>`. `std` is just another package name.

```rust
import myproject.utils
import std.collections
import raylib.core
```

## Packages

A package groups modules and plugins into one compilation unit.

```rust
package myproject
module routes
```

```rust
@package_transform(ffi_bindgen)
package myproject
plugin first
```

Rules:

- `package` is required in package entry files such as `first.jz`
- each file belongs to exactly one package
- package-level transforms attach above the package declaration target
- package identity must be explicit for package-scoped metaprogramming

## Plugins

Plugins are activated with `load` and documented in detail in [[plugins]].

```rust
load std.compiler
load myproject.auth_plugin
```

`load` includes the plugin's exported runtime elements, compile-time elements, macros,
initializers, and plugin lifecycle behavior. A plugin does not require a second `import`
for the same export surface.

`import` remains the mechanism for pure module namespaces. `load` remains the mechanism
for plugin namespaces and compile-time/plugin activation.

## Visibility

Element follows a Rust-like visibility model.

| Attribute | Visibility |
|---|---|
| none | Module-private |
| `@pub` | Fully public |
| `@pub(module)` | Visible within the module across all files |
| `@pub(project)` | Visible within the project, not external packages |

```rust
let private_helper := x -> x * 2
let @pub parse_config := path -> { ... }
let @pub(module) shared_cache := build_cache()
let @pub(project) internal_api := { ... }
```

## Package Management

External dependencies are declared in `first.jz` through the build recipe.

```rust
|> add_package("raylib")
|> add_package("https://github.com/user/pkg")
```

Manual vendoring is also supported.

```rust
|> include_module("foo", "./vendored/foo/include/", "./vendored/foo/lib/")
```

Downloaded packages sit in `.packages/` in the project root. Auto-updates require the
package manager. Manual or vendored packages have no auto-update behavior.

## Circular Imports

Circular value dependencies are compile errors.

Circular `::` type declarations are allowed through two-pass resolution.
