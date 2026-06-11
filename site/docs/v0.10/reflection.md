# Reflection

This chapter documents compiler-provided reflective values and meta-only types.

## `Type`

`Type` is a first-class compile-time value populated by the compiler. It represents
structural metadata about any type and is available as a parameter type or value.

```rust
let size_of: Type -> int = t -> t.size
let name_of: Type -> string = t -> t.name

size_of(Point)
name_of(int)
```

`Type` is distinct from `Kind`. `Type` describes concrete type metadata; `Kind` describes
type-constructor shape. See [[generics-and-kinds]].

## Universal Fields on `Type`

| Field | Return Type | Meaning |
|---|---|---|
| `.size` | `int` | Size in bytes |
| `.align` | `int` | Alignment requirement in bytes |
| `.name` | `string` | String name of the type |
| `.subelements` | `Arr<SubElementInfo>` | Structural type body metadata |
| `.variants` | `Arr<VariantInfo>` | Sum and enum variant metadata |
| `.is_expandable` | `bool` | Whether a sum root is open |
| `.with_subelements` | `Arr<SubElementInfo>` | Attached sum subelements |
| `.backing` | `Type` | Enum backing type |
| `.subelement_count` | `int` | Number of subelements when applicable |

For expandable roots, `.variants` lists variants visible in the current compilation
context and must not be treated as closed-world exhaustive data.

## Runtime Type Access

Any value exposes a `.type` property that returns its `Type` at runtime.

```rust
let s: Shape = Shape.Circle(5.0)

echo s.type.name;
echo s.type.size;

if s.type == Shape.Circle => handle_circle(s);
```

## Compile-Time Type Validation

```rust
@comptime
let assert_stack_safe: Type -> void = t -> {
    if t.size > 64 => panic "type exceeds safe stack frame size";
}

@assert assert_stack_safe(PacketHeader);

let alloc: (t: Type) -> ptr = @inline {
    allocate_bytes(it.size);
}
```

## Meta-Only Types

Element exposes compiler-facing meta-only types for compile-time execution. These types
may appear only in compile-time contexts such as `@comptime` declarations, transform
signatures, syntax macro signatures, and plugin-exported compile-time APIs.

Invalid:

```rust
let bad: Item = ...
```

Valid:

```rust
@comptime
let item_name: (item: Item) -> string = item.name
```

## Core Meta-Only Types

`TokenStream` is the raw token representation used by Phase-1 syntax macros.

```rust
@macro
let sql!: (stream: TokenStream) -> TokenStream = { ... }
```

`Code` is an untyped AST-backed code fragment used for quotation, splicing, and generated
output in compile-time APIs.

```rust
@comptime
let snippet: Code = {
    quote! { let x := 10 }
}
```

`Item` is a typed handle to one declaration-level AST element. Likely properties include
`item.name`, `item.kind`, `item.attributes`, `item.span`, `item.type`, and
`item.subelements`.

`Module` is a typed handle to a module AST. Likely properties include `module.name`,
`module.package`, `module.items`, `module.imports`, `module.attributes`, and
`module.span`.

`Package` is a typed handle to the package compilation unit. Likely properties include
`package.name`, `package.modules`, `package.plugins`, `package.entry`, and
`package.attributes`.

`SubElementInfo` describes one subelement inside a type-like declaration. Stored
subelements may expose offset, default-value, and initialization-dependency metadata.

`AttributeInfo` describes one applied attribute instance.

```rust
if let .Some(cfg) := subelement.attributes.get(inspect) {
    let args: inspect.Args = cfg.args;
}
```

`Span` identifies source location for diagnostics and source mapping.

`Diagnostic` is a structured compile-time diagnostic value.

`Patch<T>` values describe replacements, generated code, and diagnostics.

```rust
def Patch<T> :: prod {
    replacement: Option<T>,
    emitted: Arr<Code>,
    diagnostics: Arr<Diagnostic>,
}

@comptime
let generate_inspector: (item: Item) -> Patch<Item> = { ... }
```

## Meta Types and `Kind`

Meta-only types do not participate in normal runtime type usage.

- `Type` describes ordinary language types.
- `Kind` describes type-constructor shape.
- Meta-only types belong to the compile-time meta layer.

Declared attributes follow the same boundary:

- an attribute schema such as `inspect` is not a normal runtime value
- `inspect.Args` is an ordinary type
- applied attributes are metadata, not part of runtime type identity
