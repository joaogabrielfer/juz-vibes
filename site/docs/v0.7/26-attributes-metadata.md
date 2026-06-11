## 26. Attributes & Metadata

Attributes are typed compile-time metadata schemas. They are declared explicitly and may
then be applied to supported declaration targets.

### Declared Attributes

Use `@attribute` to declare a typed attribute schema.

```rs
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

A declared attribute specifies:
- valid target kinds,
- a typed argument payload schema.

Applied usage:

```rs
def PlayerComponent :: prod {
    hp: int @inspect(min = 0, max = 100),
    speed: float @inspect(min = 0.0),
}
```

Rules:
- attribute arguments use named assignment syntax,
- unknown keys are compile errors,
- invalid argument types are compile errors,
- missing required keys are compile errors,
- attributes may only be applied to declared target kinds.

### Target Kinds

The initial target vocabulary includes:
- `.subelement`
- `.type`
- `.variant`
- `.let`
- `.param`
- `.module`
- `.package`

Additional target kinds may be introduced later.

### Attribute Schemas and Payload Types

An attribute schema symbol such as `inspect` is not a normal runtime value. Its generated
argument payload type is an ordinary type available to compile-time reflection APIs.

Example:

```rs
let cfg: inspect.Args = ...
```

The attribute schema itself remains part of the compile-time metadata layer.

### Attributes as Notes

Declared attributes replace ad hoc string-note systems for stable language and library
features. Instead of querying loose note names, compile-time code should query declared
attribute schemas.

Example:

```rs
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

Compile-time code can then inspect `serialize` directly instead of matching a raw note
string.

### Typed Reflection

Applied attributes are reflected through `AttributeInfo` and typed payload accessors.

```rs
@comptime
let has_inspect: (SubElementInfo) -> bool = subelement -> {
    subelement.attributes.has(inspect)
}

@comptime
let inspect_bounds: (SubElementInfo) -> Option<(Option<float>, Option<float>)> =
    subelement -> {
        if let cfg := subelement.attributes.get(inspect) {
            let args: inspect.Args = cfg.args;
            .Some((args.min, args.max))
        } else {
            .None
        }
    }
```

### `@meta`

`@meta` is an untyped metadata escape hatch for experimental, local, or tool-specific
annotations.

```rs
@meta(ui.widget = "slider")
@meta(editor.color = "red")
```

`@meta` is not the primary attribute mechanism. Stable language and library features
should prefer declared typed attributes.

### Interaction with `Type` and `Kind`

Attributes do not participate in runtime type identity.

- the schema symbol such as `inspect` is not a runtime value,
- `inspect.Args` is an ordinary type,
- applied `@inspect(...)` metadata is compile-time metadata only.

`Kind` continues to classify type constructors. Attribute schemas are not ordinary kinded
runtime type constructors.
