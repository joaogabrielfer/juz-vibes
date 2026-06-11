## 8. The Type Primitive & Reflection

`Type` is a first-class compile-time value populated by the compiler. It represents
structural metadata about any type and is available as a parameter type or value. `Type`
does not conflict with the `def` keyword.

`Type` is distinct from `Kind`. `Type` describes concrete type metadata; `Kind` describes
type-constructor shape. See [[25-kinds-higher-kinded-types]].

```rs
let size_of: (Type) -> int = t -> t.size
let name_of: (Type) -> string = t -> t.name

size_of(Point)
name_of(int)
```

### Universal Fields on `Type`

| Field | Return Type | Meaning |
|---|---|---|
| `.size` | `int` | Size in bytes |
| `.align` | `int` | Alignment requirement in bytes |
| `.name` | `string` | String name of the type |
| `.subelements` | `Arr<SubElementInfo>` | For `prod`, `union`, and other structural type bodies: names, kinds, types, and metadata |
| `.variants` | `Arr<VariantInfo>` | For `sum` and `enum`: variant names and payload types |
| `.is_expandable` | `bool` | Whether a `sum` root is open (`expandable sum`) |
| `.with_subelements` | `Arr<SubElementInfo>` | Attached subelements declared with `sum ... with {}` |
| `.backing` | `Type` | For `enum<T>`: the integer backing type |
| `.subelement_count` | `int` | Number of subelements when applicable |

For expandable roots, `.variants` lists variants visible in the current compilation
context and must not be treated as closed-world exhaustive data.

### Runtime Type Access on Values

Any value exposes a `.type` property that returns its `Type` at runtime:

```rs
let s: Shape = .Circle { r: 5.0 }

echo s.type.name
echo s.type.size

if s.type == Shape.Circle => handle_circle(s)
```

### Compile-Time Type Validation

```rs
@comptime
let assert_stack_safe: (Type) -> void = t -> {
    if t.size > 64 => panic "type exceeds safe stack frame size";
}

@assert assert_stack_safe(PacketHeader)

let alloc: (t: Type) -> ptr = @inline {
    allocate_bytes(it.size)
}

let p := alloc(Point)
let n := alloc(u64)
```

### Meta-Only Types

Element exposes compiler-facing meta-only types for compile-time execution. These types
may appear only in compile-time contexts such as `@comptime` declarations, transform
signatures, syntax macro signatures, and plugin-exported compile-time APIs.

Using a meta-only type in a runtime declaration is a compile error.

Invalid:

```rs
let bad: Item = ...
```

Valid:

```rs
@comptime
let item_name: (Item) -> string = item -> item.name
```

### Core Meta-Only Types

#### `TokenStream`

`TokenStream` is the raw token representation used by Phase-1 syntax macros.

```rs
@macro
let sql!: (TokenStream) -> TokenStream = stream -> { ... }
```

`TokenStream` contains token-level structure only. It does not contain type information
or typed AST structure.

#### `Code`

`Code` is an untyped AST-backed code fragment used for quotation, splicing, and generated
output in compile-time APIs.

```rs
@comptime
let snippet: () -> Code = {
    quote! { let x := 10 }
}
```

`Code` is untyped by default.

#### `Item`

`Item` is a typed handle to one declaration-level AST element.

Likely properties include:
- `item.name`
- `item.kind`
- `item.attributes`
- `item.span`
- `item.type`
- `item.subelements`

#### `Module`

`Module` is a typed handle to a module AST.

Likely properties include:
- `module.name`
- `module.package`
- `module.items`
- `module.imports`
- `module.attributes`
- `module.span`

#### `Package`

`Package` is a typed handle to the package compilation unit.

Likely properties include:
- `package.name`
- `package.modules`
- `package.plugins`
- `package.entry`
- `package.attributes`

#### `SubElementInfo`

`SubElementInfo` describes one subelement inside a type-like declaration.

Likely properties include:
- `subelement.name`
- `subelement.kind`
- `subelement.type`
- `subelement.attributes`
- `subelement.span`

Stored subelements may additionally expose:
- `subelement.offset`
- `subelement.default_value`

Computed subelements may additionally expose:
- `subelement.body`
- `subelement.dependencies`

#### `AttributeInfo`

`AttributeInfo` describes one applied attribute instance.

Likely properties include:
- `attribute.name`
- `attribute.schema`
- `attribute.args`
- `attribute.span`

Typed attribute access example:

```rs
if let cfg := subelement.attributes.get(inspect) {
    let args: inspect.Args = cfg.args;
}
```

#### `Span`

`Span` identifies source location for diagnostics and source mapping.

#### `Diagnostic`

`Diagnostic` is a structured compile-time diagnostic value. It may represent warnings,
errors, notes, and help messages.

#### `Patch<T>`

Transforms return `Patch<T>` values that describe replacements, generated code, and
diagnostics.

```rs
def Patch<T> :: prod {
    replacement: Option<T>,
    emitted: Arr<Code>,
    diagnostics: Arr<Diagnostic>,
}
```

Example:

```rs
@comptime
let generate_inspector: (Item) -> Patch<Item> = item -> { ... }
```

### Meta Types and `Kind`

Meta-only types do not participate in normal runtime type usage. They are compiler/meta
types, not ordinary runtime-denotable values.

`Type` describes ordinary language types.  
`Kind` describes type-constructor shape.  
Meta-only types belong to the compile-time meta layer.

Declared attributes follow the same boundary:
- an attribute schema such as `inspect` is not a normal runtime value,
- `inspect.Args` is an ordinary type,
- applied attributes are metadata, not part of runtime type identity.
