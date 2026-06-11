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
| `.fields` | `Arr<FieldInfo>` | For `prod` types: names, types, and byte offsets |
| `.variants` | `Arr<VariantInfo>` | For `sum` and `enum`: variant names and payload types |
| `.backing` | `Type` | For `enum<T>`: the integer backing type |
| `.field_count` | `int` | Number of fields or variants |

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
