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
def Callback :: int -> string
def Display  :: trait { show: Self -> string }
```

The retired `type Foo = ...` and `def Foo = ...` forms are invalid.

## Product Types

Product types are named structs. Stored subelements determine runtime layout and the
generated constructor shape. Runtime size is the sum of all stored subelement sizes plus
alignment padding.

```rust
def User :: prod {
    id: int,
    mut name: string,
    email: string,
    show: string = "{self.name} <{self.email}>",
}

let mut u := User(1, "Alice", "alice@example.com")
let named := User(.id = 2, .name = "Bea", .email = "bea@example.com")
u.name = "Bob"
```

## Subelements

`subelement` is the general term for named members declared inside a type body.

A product subelement may use any of these forms:

- `name: T`
- `mut name: T`
- `name: T = expr`
- `mut name: T = expr`
- `name := expr`
- `mut name := expr`

These forms define ordinary structural subelements. A subelement without a default is a
required constructor argument. A subelement with a default is initialized during
construction and then stored in the value.

`field` remains acceptable for storage-only discussion, but `subelement` is the preferred
general term because type bodies may also declare members with self-derived default
initializers.

## Constructors

Product and payload-carrying variant constructors are callable elements generated from
type declarations.

```rust
def Point :: prod {
    x: int,
    y: int,
}

let p1 := Point(10, 20)
let p2 := Point(.x = 10, .y = 20)
```

The generated product constructor is equivalent to a public callable constructor that
uses primitive structural construction.

```rust
let Point: (x: int, y: int) -> Self = {
    Self { x, y }
}
```

Users may replace the generated public constructor by declaring a constructor with the
same type name.

```rust
let Point: (x: int = 0, y: int = 0) -> Self = {
    Self { x, y }
}

let origin := Point()
let p := Point(.y = 5)
```

Inside a constructor declaration, `Self` is bound to the constructed type or variant.
`Self { ... }` performs primitive structural construction and bypasses public constructor
lookup. Calling `Point(...)` inside the `Point` constructor calls the public constructor
and may recurse.

```rust
// Invalid: recursively calls the public constructor.
let Point: (x: int = 0, y: int = 0) -> Self = {
    Point(x, y)
}
```

`Self()` is not used for primitive construction because call syntax denotes public
constructor/function calls.

Constructors support named arguments, default arguments, and explicit partial
application in the same way as ordinary functions.

```rust
let on_x_axis := Point(.x = _, .y = 0)
let p := on_x_axis(10)
```

## Sum Types

Sum types are tagged unions. Exactly one variant is active at runtime. Size is the
discriminant tag plus the largest variant size.

```rust
def Shape :: sum {
    Circle(float),
    Rect { w: float, h: float },
    Point,
}

let c: Shape = Shape.Circle(5.0)
let r: Shape = Shape.Rect(.w = 10.0, .h = 20.0)
let p: Shape = Shape.Point
```

Payload-carrying variants have generated callable constructors. Nullary variants are
values and do not require call syntax.

```rust
let circle_of := Shape.Circle(_)
let rect_with_w_1 := Shape.Rect(.w = 1.0, .h = _)
```

A custom variant constructor may replace the generated public constructor.

```rust
let Shape.Circle: (radius: float = 0.0) -> Self = {
    Self { radius + 10.0 }
}
```

The exact stored-field projection syntax for tuple-like variant payloads is specified by
the pattern and reflection surfaces. Named product-like variants expose their declared
subelement names.

## Expandable Sums

`expandable sum` declares an open sum family. New members can be declared later as nested
types under the root.

```rust
def CompilerError :: expandable sum

def CompilerError::Generic :: string

def CompilerError::Http :: prod {
    code: int,
    message: string,
}
```

Constructors under the expandable root produce values of the root type.

```rust
let e1: CompilerError = CompilerError.Generic("bad input")
let e2: CompilerError = CompilerError.Http(code = 404, message = "not found")
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

`|` in type position denotes a safe union type. Anonymous union types are valid only in
input positions. A function return type must not be an anonymous union. A function that
can produce different result shapes must return a named `sum`, `expandable sum`, or
standard carrier such as `Fallible<T>`.

```rust
let overloaded :: (string | int) -> int

// Invalid:
let parse :: string -> int | float | Error

// Valid:
def ParsedNumber :: sum { Int(int), Float(float) }
let parse :: string -> ParsedNumber
```

For `[]` pattern-matched functions, the input union type is inferred from arm patterns if
no `::` declaration is provided. Arms must agree on return type.

## Type Aliases

```rust
def Byte    :: u8
def UserId  :: int
def Matrix  :: Arr<Arr<float>>
def Handler :: Request -> Response
```

## Refinement Types

Refinement types are subset types over an existing base type. The predicate is written
with `where` and refers to the candidate value as `it`.

```rust
def Nat  :: int where it >= 0
def Port :: int where it >= 0 && it <= 65535
def Even :: int where it % 2 == 0
```

A value of a refinement type may be used where its base type is expected. A value of the
base type may be used as the refinement type only when the compiler can prove the
predicate or after explicit validation at a construction boundary.

Proofs are intentionally lightweight in this revision. The compiler should accept obvious
literal, constant, range-check, and guard-based proofs, but it should not require a
general-purpose theorem solver.

```rust
let @const NUMBER_OF_PEOPLE := 100

def PersonId :: int where it > 0 && it <= NUMBER_OF_PEOPLE
```

## Array Type Expressions

Fixed-size arrays are language-level type expressions.

```rust
let coords :: int[3]

def PersonId :: enum<usize> {
    John,
    Paul,
    Greg,
}

let phone_numbers :: string[3: PersonId]
```

`T[N]` is a fixed-size array of `T` with compile-time length `N`. `T[N: I]` is a
fixed-size array of `T` with compile-time length `N` and logical index domain `I`.

The custom index domain `I` must be finite and must contain exactly `N` representable
values. Fixed arrays are total over their declared index domain.

Dynamic arrays are standard-library declarations owned by [[std-collections]].

```rust
let names   :: Arr<string>
let queued  :: Arr<string>[PersonId]
```

`Arr<T>` is a dynamic growable array with the default runtime index domain. `Arr<T>[I]`
is a bounded dynamic array whose possible indices are drawn from `I`.

## Standard Types

Concrete carriers such as `Fallible<T>`, `Either<L, R>`, `Option<T>`, and collections are
standard-library declarations. Their declaration surfaces live in [[std-core-types]] and
[[std-error]].

## Primitive Types

`int`, `float`, `bool`, `string`, `char`, `unit`, `never`, `usize`, `isize`, `u8`,
`u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`, and `ptr` are primitive
type names.

`unit` is the one-value type. It is used for empty argument lists and for successful
operations with no meaningful payload. In value position, `unit` denotes the sole value
of type `unit`.

In type position, `unit -> T` denotes a callable zero-argument element returning `T`. A
plain `T` annotation declares a value-like zero-argument element instead.

`never` is the zero-value type. It is used for declarations such as `panic` and `exit`
that do not return normally.

## Numeric Types and Literals

`int` and `float` are stable aliases, not target-dependent aliases.

```rust
def int   :: i64
def float :: f64
```

`usize` and `isize` are the target pointer-sized integer types. They are used for sizes,
offsets, and low-level indexing surfaces where target address width matters.

`Byte` is the standard alias for `u8`. The language should not introduce broad aliases
such as `Size`; use `usize` directly when pointer-sized behavior is intended.

Integer literals begin as exact compile-time integers. They are coerced into a concrete
numeric type by context. When no context is available, an integer literal defaults to
`int`.

```rust
let a: i8 = 10
let b: u8 = 255
let c := 10

// Invalid: 256 is outside u8.
let d: u8 = 256
```

Float literals begin as exact compile-time float literals. They are coerced into a
concrete float type by context. When no context is available, a float literal defaults to
`float`.

```rust
let a: f32 = 1.5
let b: f64 = 1.5
let c := 1.5
```

Literals may be assigned to refinement types when the compiler can prove the predicate.

```rust
def Port :: u16 where it > 0

let http: Port = 80
// Invalid:
let missing: Port = 0
```

Implicit numeric conversion between already-typed values is not part of this revision.
Use explicit widening, checked conversion, or validation APIs when converting between
concrete numeric types.

The exact overflow behavior and the standard conversion API remain open.
