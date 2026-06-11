# Functions and Lambdas

This chapter owns function definition syntax, implicit arguments, function types,
generic function signatures, default arguments, partial application, and return forms.

## Universal Function Form

The canonical shape is a grammar sketch, not literal Element code:

```txt
let [mut] [attrs] name: <generics> arg_types -> return_type = body
```

## Syntax Gradient

```rust
let double := x -> x * 2

let add := (x, y) -> x + y

let double: int -> int = { it * 2 } //here `it` is an inferred argument if the function has only one arg and the type is written

let offset: (base: int, scale: int) -> int = {
    base + scale * 16
}

let process :: int -> int;
let process = x -> {
    let doubled  := x * 2;
    let adjusted := doubled + 16;
    adjusted
}

let identity: <T> T -> T = { it }

let show_if: <T: Display> (cond: bool, val: T) -> string =  {
    if cond => Display::show(val)
    else    => ""
}
```

Inline function annotations may bind parameter names by wrapping the parameter list in
parentheses.

```rust
let add: (a: int, b: int) -> int = {
    a + b
}
```

An inline function annotation that binds parameter names supplies those names to the
body. A body written as a lambda may instead be paired with a standalone `::` signature.

```rust
let add :: int, int -> int,
let add = a, b -> a + b
```

A function cannot annotate the types and have the args bind in the body at the same time. Basically, a declaration cannot have 2 `->` arrows at once. It either binds the args in the types declaration, or binds them in the body of a type inferred function
```rust
//ALLOWED
let foo := x, y -> ...

//ALLOWED
let foo: (x: int, y: int) -> int = ...

//NOT ALLOWED
let foo: (int, int) -> int = x, y -> ...
```

## Implicit Returns

The final expression in a function body is the return value.

```rust
let add := (x, y) -> x + y

let add := (x, y) -> {
    x + y
}
```

## `return` and `be`

`be` is a syntactic alternative to `return` where there are no arguments in for the element. They are semantically identical and both
require a semicolon.

```rust
//Wrong usage
let add := (x, y) -> {
    be x + y;
}

//Correct
let foo := {
	let bar := 10;
	be bar + 5;
}

//Is the same as:
let foo := {
	let bar := 10;
	bar + 5
}
```

Implicit returns remain idiomatic. `be` exists for readability in places where `return`
inside a constant-like binding reads poorly.

The exact scope where `be` is stylistically preferred is tracked in [[open-questions]].

## `it`

`it` is available only when the argument type is known from the annotation. Without an
annotation, an explicit argument name with `->` is required.

```rust
let double: int -> int       = { it * 2 }
let greet:  string -> string = { "Hello, " <> it }
let negate: bool -> bool     = { !it }

// Invalid:
let f := it * 2

// Valid:
let f := x -> x * 2
```

## `~` Lambda Argument

`~` in an argument list, left of `->`, denotes one unnamed argument that flows implicitly
as the input to the right-side composition. At most one `~` may appear in an argument
list.

```rust
let map = ~, fn -> for_each(fn) >> collect
let negate_all = ~ -> map(negate)
```

`~` in expression position remains bitwise NOT notation.

```rust
let mask := ~flags
```

## Function Types

Named function type aliases use `def` with `::` and the ordinary function type syntax.

```rust
def BinaryIntOp  :: int, int -> int;
def Predicate<T> :: T -> bool;
def Transformer  :: string -> string;
```

Top-level function types are written without argument-list parentheses.

```rust
let add :: int, int -> int;
let negate :: bool -> bool;
let now :: void -> Time;
```

`void -> T` declares a callable zero-argument element returning `T`. A plain `T`
annotation declares a value-like zero-argument element. These forms are not equivalent:
`let now: Time = ...` is accessed as `now` and may use first-access caching when pure,
while `let now: void -> Time = ...` is invoked as `now()` and never receives bare-access
caching.

```rust
let now: Time = current_time()
let now: void -> Time = current_time()
```

Nested function types use parentheses to make arrow grouping explicit.

```rust
let apply :: int, (int -> int) -> int;
let apply = n, f -> f(n)

let map :: <T> Arr<T>, (T -> T) -> Arr<T>;
```

Chained arrows without parentheses are invalid when more than one parse would be
possible.

```rust
// Invalid:
let compose :: int -> int -> int;
```

Use parentheses for the nested function input or output instead.

```rust
let returns_fn :: int -> (int -> int);
let takes_fn   :: (int -> int) -> int;
```

## Default Arguments

Default arguments may appear in function signatures.

```rust
let add_or_inc_1: (a: int, b: int = 1) -> int = {
    a + b
}

assert 5 == add_or_inc_1(4)
assert 5 == add_or_inc_1(4, 1)
assert 5 == add_or_inc_1(.b = 4, .a = 1)
```

An omitted defaulted parameter uses its default. Omitting a required parameter is a type
error.

```rust
// Invalid:
add_or_inc_1(.b = 3)
```

Required parameters must precede defaulted parameters. A later version may permit
required parameters after defaults when all following arguments are named, but the v0.11
surface keeps the rule simple.

## Partial Application

Using `_` in a regular function call creates a closure waiting for that argument.
Missing required arguments without `_` are errors, not implicit partial applications.

```rust
let inc_3 := add_or_inc_1(_, 3)
let inc_3_named := add_or_inc_1(.a = _, .b = 3)

let add_to_5 := add_or_inc_1(5, _)
let add_to_5_named := add_or_inc_1(.a = 5, .b = _)

assert 6 == add_or_inc_1(5)
assert 8 == add_to_5(3)
```

Bare access to a partially applied function does not call it.

```rust
// Invalid:
assert 6 == add_to_5
```
