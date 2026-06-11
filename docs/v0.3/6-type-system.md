## 6. Type System

### The `def` Keyword

`def` is the universal type declaration keyword, replacing the retired `type` keyword. It
covers all type forms: product types, sum types, enums, unions, aliases, type traits, and
`extend` declarations:

```rs
def Point    = prod { x: int, y: int };
def Shape    = sum  { Circle { r: float }, Rect { w: float, h: float } };
def Status   : enum<int> { Active = 1, Inactive, Pending }
def UserId   = int;
def Callback = def (int) -> string;
```

### Product Types (`prod {}`)

Named structs. All fields are always present. Runtime size is the sum of all field sizes
(plus any alignment padding):

```rs
def User = prod {
    id:    int,
    name:  string,
    email: string,
};

let mut u := User(1, "Alice", "alice@example.com");
u.name = "Bob";    // valid — u is mut
```

### Sum Types (`sum {}`)

Tagged unions. Exactly one variant is active at runtime. Size is the discriminant tag plus
the largest variant's size:

```rs
def Shape = sum {
    Circle { r: float },
    Rect   { w: float, h: float },
    Point,
};

def Result: <T, E> = sum {
    Ok(T),
    Err(E),
};

// Variant construction via dot-shorthand (type inferred from context):
let s: Shape = .Circle { r: 5.0 };
let r: Result<int, string> = .Ok(42);
let e: Result<int, string> = .Err("not found");
```

### C-Style Enums (`enum<T>`)

Integer-backed enumerations. Fields are implicitly sequential unless overridden. The backing
type `T` must be an integer primitive:

```rs
def Direction: enum<int> { North, South, East, West }

def Priority: enum<u8> {
    Low    = 1,
    Medium,    // 2
    High,      // 3
    Critical,  // 4
}
```

### Untagged Unions (`union`)

Unsafe raw memory overlap. All fields share the same memory. Size is the largest field.
Use only for low-level interop:

```rs
def RawValue: union {
    as_int:   int,
    as_float: float,
    as_bytes: Arr<u8>,
};
```

### Type Aliases

```rs
def UserId   = int;
def Matrix   = Arr<Arr<float>>;
def Handler  = def (Request) -> Response;
```

### Primitive Types

`int`, `float`, `bool`, `string`, `char`, `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`,
`i64`, `f32`, `f64`, `ptr`. Full numeric tower specification pending.

---

