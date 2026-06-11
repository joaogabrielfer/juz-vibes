## 6. Type System

### The `def` Keyword

`def` is the universal type declaration keyword. It covers product types, sum types,
enums, raw unions, aliases, function type aliases, traits, and `extends` declarations.
Every `def` declaration uses `::`.

```rs
def Point    :: prod { x: int, y: int }
def Shape    :: sum  { Circle { r: float }, Rect { w: float, h: float } }
def Status   :: enum<int> { Active = 1, Inactive, Pending }
def UserId   :: int
def Callback :: def((int) -> string)
def Display  :: trait { show: (Self) -> string }
```

The retired `type Foo = ...` and `def Foo = ...` forms are invalid.

### Product Types (`prod {}`)

Product types are named structs. All fields are always present. Runtime size is the sum
of all field sizes plus alignment padding.

```rs
def User :: prod {
    id:    int,
    name:  string,
    email: string,
}

let mut u := User(1, "Alice", "alice@example.com")
u.name = "Bob"
```

### Sum Types (`sum {}`)

Sum types are tagged unions. Exactly one variant is active at runtime. Size is the
discriminant tag plus the largest variant size.

```rs
def Shape :: sum {
    Circle { r: float },
    Rect   { w: float, h: float },
    Point,
}

def Result<T, E> :: sum {
    Ok(T),
    Err(E),
}

let s: Shape = .Circle { r: 5.0 }
let r: Result<int, string> = .Ok(42)
let e: Result<int, string> = .Err("not found")
```

### C-Style Enums (`enum<T>`)

Integer-backed enumerations. Fields are implicitly sequential unless overridden. The
backing type `T` must be an integer primitive.

```rs
def Direction :: enum<int> { North, South, East, West }

def Priority :: enum<u8> {
    Low    = 1,
    Medium,
    High,
    Critical,
}
```

### Raw Memory Unions (`union`)

`union` declares unsafe raw memory overlap. All fields share the same memory. Size is the
largest field. Use only for low-level interop.

```rs
def RawValue :: union {
    as_int:   int,
    as_float: float,
    as_bytes: Arr<u8>,
}
```

### Union Types in Signatures

`|` in type position denotes a safe union type. This is unambiguous because type contexts
after `::` or `:` do not contain bitwise OR expressions.

```rs
let overloaded :: (string | int) -> int
let parse      :: (string) -> int | float | Err
```

For `[]` pattern-matched functions, the input union type is inferred from arm patterns if
no `::` declaration is provided. Arms must agree on return type.

```rs
let stringify = [
    (n: int)    => n.show
    (f: float)  => f.show
    (s: string) => s
]
```

The inferred type is:

```rs
(int | float | string) -> string
```

### Type Aliases

```rs
def UserId  :: int
def Matrix  :: Arr<Arr<float>>
def Handler :: def((Request) -> Response)
```

### Primitive Types

`int`, `float`, `bool`, `string`, `char`, `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`,
`i64`, `f32`, `f64`, and `ptr` are primitive type names. The full numeric tower,
overflow behavior, casting rules, and literal typing remain open.
