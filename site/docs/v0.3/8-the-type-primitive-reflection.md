## 8. The Type Primitive & Reflection

`Type` (capitalized) is a first-class compile-time value populated by the compiler. It
represents structural metadata about any type and is available as a parameter type or
value. `Type` does not conflict with the `def` keyword:

```rs
let size_of: (Type) -> int = t -> t.size;
let name_of: (Type) -> string = t -> t.name;

size_of(Point)    // sizeof(Point) in bytes
name_of(int)      // "int"
```

### Universal Fields on All `Type` Values

| Field | Return Type | Meaning |
|---|---|---|
| `.size` | `int` | Size in bytes |
| `.align` | `int` | Alignment requirement in bytes |
| `.name` | `string` | String name of the type |
| `.fields` | `Arr<FieldInfo>` | For `prod` types: names, types, and byte offsets of fields |
| `.variants` | `Arr<VariantInfo>` | For `sum`/`enum`: variant names and payload types |
| `.backing` | `Type` | For `enum<T>`: the integer backing type `T` |
| `.field_count` | `int` | Number of fields (for `prod`) or variants (for `sum`/`enum`) |

### Runtime Type Access on Values

Any value exposes a `.type` property that returns its `Type` at runtime:

```rs
let s: Shape = .Circle { r: 5.0 };

echo s.type.name;              // "Circle"
echo s.type.size;              // sizeof(Circle variant)

if s.type == Shape.Circle => handle_circle(s);
```

### Compile-Time Type Validation

```rs
@comptime
let assert_stack_safe: (Type) -> void = t -> {
    if t.size > 64 => panic "Type exceeds safe stack frame size!";
};

@assert assert_stack_safe(PacketHeader);   // validated at compile time

// Generic bound using Type:
let alloc: (t: Type) -> ptr = @inline {
    allocate_bytes(it.size)
};

let p := alloc(Point);     // allocates sizeof(Point) bytes
let n := alloc(u64);       // allocates sizeof(u64) bytes
```

---

