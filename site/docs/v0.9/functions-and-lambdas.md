# Functions and Lambdas

This chapter owns function definition syntax, implicit arguments, anonymous function
types, generic function signatures, and return forms.

## Universal Function Form

The canonical shape is a grammar sketch, not literal Element code:

```txt
let [mut] [attrs] name: <generics> (args) -> return_type = body
```

## Syntax Gradient

```rust
let double := x -> x * 2

let add := (x, y) -> x + y

let double: (int) -> int = { it * 2 }

let offset: (base: int, scale: int) -> int = {
    base + scale * 16
}

let process: (int) -> int = x -> {
    let doubled  := x * 2;
    let adjusted := doubled + 16;
    adjusted
}

let identity: <T> (T) -> T = { it }

let show_if: <T: Display> (bool, T) -> string = (cond, val) -> {
    if cond => Display.show(val)
    else    => ""
}
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

`be` is a syntactic alternative to `return`. They are semantically identical and both
require a semicolon.

```rust
let add := (x, y) -> {
    be x + y;
}

let add := (x, y) -> {
    return x + y;
}
```

Implicit returns remain idiomatic. `be` exists for readability in places where `return`
inside a constant-like binding reads poorly.

The exact scope where `be` is stylistically preferred is tracked in [[open-questions]].

## `it`

`it` is available only when the argument type is known from the annotation. Without an
annotation, an explicit argument name with `->` is required.

```rust
let double: (int) -> int       = { it * 2 }
let greet:  (string) -> string = { "Hello, " <> it }
let negate: (bool) -> bool     = { !it }

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

Named function type aliases use `def` with `::`.

```rust
def BinaryIntOp  :: def((int, int) -> int)
def Predicate<T> :: def((T) -> bool)
def Transformer  :: def((string) -> string)
```

Anonymous function types in type positions use `def(...)` with the signature wrapped in a
second set of parentheses.

```rust
let apply: (int, def((int) -> int)) -> int = (n, f) -> { f(n) }
let map: <T> (Arr<T>, def((T) -> T)) -> Arr<T> = { ... }
```
