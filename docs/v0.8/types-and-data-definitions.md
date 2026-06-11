# Types and Data Definitions

`def` is the universal type declaration keyword. It covers product types, sum types,
enums, raw unions, aliases, function type aliases, traits, and `extends` declarations.
Every `def` declaration uses `::`.

```rust
def Point    :: prod { x: int, y: int }
def Shape    :: sum  { Circle { r: float }, Rect { w: float, h: float } }
def Error    :: expandable sum
def Status   :: enum<int> { Active = 1, Inactive, Pending }
def UserId   :: int
def Callback :: def((int) -> string)
def Display  :: trait { show: (Self) -> string }
```

The retired `type Foo = ...` and `def Foo = ...` forms are invalid.

## Product Types

Product types are named structs. All stored subelements are always present. Runtime size
is the sum of all stored subelement sizes plus alignment padding.

```rust
def User :: prod {
    id:    int,
    name:  string,
    email: string,
}

let mut u := User(1, "Alice", "alice@example.com")
u.name = "Bob"
```

## Subelements

`subelement` is the general term for named members declared inside a type body.

A subelement may be:

- stored
- computed
- introduced by later member-like declaration forms

`field` is no longer the preferred general term once a member may be a zero-argument
computed declaration instead of pure storage.

## Sum Types

Sum types are tagged unions. Exactly one variant is active at runtime. Size is the
discriminant tag plus the largest variant size.

```rust
def Shape :: sum {
    Circle { r: float },
    Rect   { w: float, h: float },
    Point,
}

let s: Shape = .Circle { r: 5.0 }
```

## Expandable Sums

`expandable sum` declares an open sum family. New members can be declared later as nested
types under the root.

```rust
def CompilerError :: expandable sum

def CompilerError.Generic :: string

def CompilerError.Http :: prod {
    code: int,
    message: string,
}
```

Constructors under the expandable root produce values of the root type.

```rust
let e1: CompilerError = CompilerError.Generic("bad input")
let e2: CompilerError = CompilerError.Http(code: 404, message: "not found")
```

Pattern matching on an expandable root must include `_`.

## Attached Sum Subelements

`with {}` declares stored subelements attached to every value of a sum. Attached
subelements are not variants and are not matched as cases.

```rust
def Error :: expandable sum with {
    meta: prod {
        location: Option<SourceLoc>,
        notes: Arr<string>,
        frames: Arr<ErrorFrame>,
    } = (location: .None, notes: [], frames: []),
}
```

Constructor calls may pass attached subelements by name. If omitted, default values are
used.

```rust
let e: Error = Error.Generic(
    "bad token",
    meta: (
        location: .Some(SourceLoc("main.el", 2, 7)),
        notes: ["during parse"],
        frames: [],
    ),
)
```

`with {}` is currently defined for `sum` and `expandable sum` only.

## C-Style Enums

Integer-backed enumerations. Variants are implicitly sequential unless overridden. The
backing type must be an integer primitive.

```rust
def Direction :: enum<int> { North, South, East, West }

def Priority :: enum<u8> {
    Low    = 1,
    Medium,
    High,
    Critical,
}
```

## Raw Memory Unions

`union` declares unsafe raw memory overlap. All stored subelements share the same memory.
Size is the largest stored subelement. Use only for low-level interop.

```rust
def RawValue :: union {
    as_int:   int,
    as_float: float,
    as_bytes: Arr<u8>,
}
```

## Union Types in Signatures

`|` in type position denotes a safe union type. This is unambiguous because type contexts
after `::` or `:` do not contain bitwise OR expressions.

```rust
let overloaded :: (string | int) -> int
let parse      :: (string) -> int | float | Error
```

For `[]` pattern-matched functions, the input union type is inferred from arm patterns if
no `::` declaration is provided. Arms must agree on return type.

## Type Aliases

```rust
def UserId  :: int
def Matrix  :: Arr<Arr<float>>
def Handler :: def((Request) -> Response)
```

## Standard Types

Concrete carriers such as `Fallible<T>`, `Either<L, R>`, `Option<T>`, and collections are
standard-library declarations. Their declaration surfaces live in [[std-core-types]] and
[[std-error]].

## Primitive Types

`int`, `float`, `bool`, `string`, `char`, `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`,
`i64`, `f32`, `f64`, and `ptr` are primitive type names.

The numeric tower, overflow behavior, casting rules, and literal typing remain open.
