## 22. Modules, Packages & Visibility

### Modules

Modules are pure namespaces imported with `import`. Importing a module has no runtime side
effects.

```rs
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

Each file declares its module at the top:

```rs
module foo
```

Directory organization is idiomatic and recommended but not enforced.

Local modules use `<package_name>.<module_name>`. `std` is just another package name.
Custom packages follow the same convention.

```rs
import myproject.utils
import std.collections
import raylib.core
```

### Packages

A package groups modules and plugins into one compilation unit.

A file may declare its package with:

```rs
package myproject
```

Examples:

```rs
package myproject
module routes
```

```rs
@package_transform(ffi_bindgen)
package myproject
plugin first
```

Rules:
- `package` is required in package entry files such as `first.jz`,
- each file belongs to exactly one package,
- package-level transforms attach above the package declaration target,
- package identity must be explicit for package-scoped metaprogramming.

### Plugins

Plugins are modules with lifecycle hooks. They are loaded with `load`, can define `ini`
declarations, and can execute compile-time code.

```rs
load std.compiler
load myproject.auth_plugin
```

A file declares itself a plugin with `plugin Name` at the top:

```rs
package myproject
plugin first
```

`load` includes the plugin's exported runtime elements, compile-time elements, macros,
initializers, and plugin lifecycle behavior. A plugin does not require a second `import`
for the same export surface.

`import` remains the mechanism for pure module namespaces. `load` remains the mechanism
for plugin namespaces and compile-time/plugin activation.

### Visibility

Element follows a Rust-like visibility model.

| Attribute | Visibility |
|---|---|
| none | Module-private |
| `@pub` | Fully public |
| `@pub(module)` | Visible within the module across all files |
| `@pub(project)` | Visible within the project, not external packages |

```rs
let private_helper := x -> x * 2
let @pub parse_config := path -> { ... }
let @pub(module) shared_cache := build_cache()
let @pub(project) internal_api := { ... }
```

### Package Management

External dependencies are declared in `first.jz` through the build recipe.

```rs
|> add_package("raylib")
|> add_package("https://github.com/user/pkg")
```

Manual vendoring is also supported:

```rs
|> include_module("foo", "./vendored/foo/include/", "./vendored/foo/lib/")
```

Downloaded packages sit in `.packages/` in the project root. Auto-updates require the
package manager. Manual or vendored packages have no auto-update behavior.

### Circular Imports

Circular value dependencies are compile errors.

Circular `::` type declarations are allowed through two-pass resolution.
