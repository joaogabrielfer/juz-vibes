# Attributes and Metadata

Attributes are typed compile-time metadata schemas. They are declared explicitly and may
then be applied to supported declaration targets.

## Declared Attributes

Use `@attribute` to declare a typed attribute schema.

```rust
@attribute
def inspect :: attr {
    targets: [.subelement],
    args: prod {
        min: Option<float>,
        max: Option<float>,
        step: Option<float>,
        label: Option<string>,
    }
}
```

A declared attribute specifies valid target kinds and a typed argument payload schema.

Applied usage:

```rust
def PlayerComponent :: prod {
    hp: int @inspect(min = 0, max = 100),
    speed: float @inspect(min = 0.0),
}
```

Rules:

- attribute arguments use named assignment syntax
- unknown keys are compile errors
- invalid argument types are compile errors
- missing required keys are compile errors
- attributes may only be applied to declared target kinds

## Target Kinds

The initial target vocabulary includes:

- `.subelement`
- `.type`
- `.variant`
- `.let`
- `.param`
- `.module`
- `.package`

Additional target kinds may be introduced later.

## Language-Defined Attributes

This inventory helps readers and tooling find the subsystem that owns each attribute.

| Attribute | Primary target | Purpose |
|---|---|---|
| `@attribute` | `def ... :: attr` | Declares a typed attribute schema |
| `@meta(...)` | declarations | Untyped experimental metadata |
| `@pub` | declarations | Fully public visibility |
| `@pub(module)` | declarations | Visible across files in the same module |
| `@pub(project)` | declarations | Visible within the project/package boundary |
| `@impl(...)` | `extends` blocks, `let` | Declares a trait implementation |
| `@auto_impl(...)` | type declarations | Requests derived trait implementations |
| `@requires(...)` | expandable families | Requires traits on later nested members |
| `@override(...)` | implementations | Resolves same-level implementation conflicts |
| `@memoize` | pure declarations | Caches pure function results |
| `@pure` | functions/elements | Checks that the visible effect set is empty |
| `@effects(...)` | functions/elements | Declares visible effects performed by a declaration |
| `@handles(...)` | handlers | Declares effects consumed by a handler |
| `@recoverable(...)` | `Error` members | Declares the repair response type for an error variant |
| `@notation` | `let` or effect operation | Enables notation syntax |
| `@position(...)` | notation declarations | Declares prefix, infix, or postfix placement |
| `@bind(...)` | notation declarations | Maps notation operands to parameters |
| `@assoc(...)` | infix/postfix notation | Declares associativity |
| `@precedence(...)` | notation declarations | Declares precedence level |
| `@adjacent(...)` | notation declarations | Allows no-whitespace adjacency |
| `@macro` | macros | Declares syntax macro behavior |
| `@comptime` | declarations | Marks compile-time callable code |
| `@test` | lowered test declarations | Marks a unit test for the test harness |
| `@transform` | items | Applies an item transform |
| `@module_transform(...)` | module declarations | Applies a module transform |
| `@package_transform(...)` | package declarations | Applies a package transform |
| `@align(n)` | low-level types/subelements | Requests memory alignment |
| `@packed` | product/interop layouts | Requests no struct padding |

Additional standard-library attributes may be declared with `@attribute`; they are not
automatically language-defined.

## Attribute Schemas and Payload Types

An attribute schema symbol such as `inspect` is not a normal runtime value. Its generated
argument payload type is an ordinary type available to compile-time reflection APIs.

```rust
let cfg: inspect.Args = ...
```

The attribute schema itself remains part of the compile-time metadata layer.

## Attributes as Notes

Declared attributes replace ad hoc string-note systems for stable language and library
features.

```rust
@attribute
def serialize :: attr {
    targets: [.subelement],
    args: prod {}
}

def Player :: prod {
    name: string @serialize(),
    hp: int @serialize(),
    secret_id: int,
}
```

Compile-time code can inspect `serialize` directly instead of matching a raw note string.

## Typed Reflection

Applied attributes are reflected through `AttributeInfo` and typed payload accessors.

```rust
@comptime
let has_inspect: (subelement: SubElementInfo) -> bool = {
    subelement.attributes.has(inspect)
}

@comptime
let inspect_bounds: (subelement: SubElementInfo) -> Option<(Option<float>, Option<float>)> = {
        if let .Some(cfg) := subelement.attributes.get(inspect) {
            let args: inspect.Args = cfg.args;
            .Some((args.min, args.max))
        } else {
            .None
        }
}
```

## `@meta`

`@meta` is an untyped metadata escape hatch for experimental, local, or tool-specific
annotations.

```rust
@meta(ui.widget = "slider")
@meta(editor.color = "red")
```

`@meta` is not the primary attribute mechanism. Stable language and library features
should prefer declared typed attributes.

## Interaction with Type and Kind

Attributes do not participate in runtime type identity.

- the schema symbol such as `inspect` is not a runtime value
- `inspect.Args` is an ordinary type
- applied `@inspect(...)` metadata is compile-time metadata only

`Kind` continues to classify type constructors. Attribute schemas are not ordinary kinded
runtime type constructors.
