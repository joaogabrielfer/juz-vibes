## 9. Function Syntax & Lambdas

### Universal Function Form

```txt
let [mut] [attrs] name: <generics> (args) -> return_type = body
```

### Syntax Gradient

```rs
// Fully inferred, named argument:
let double := x -> x * 2

// Fully inferred, multi-argument:
let add := (x, y) -> x + y

// Typed, implicit `it`:
let double: (int) -> int = { it * 2 }

// Named arguments in signature:
let offset: (base: int, scale: int) -> int = { base + scale * 16 }

// Full body with statements:
let process: (int) -> int = x -> {
    let doubled  := x * 2;
    let adjusted := doubled + 16;
    adjusted
}

// Generic:
let identity: <T> (T) -> T = { it }

// Bounded generic:
let show_if: <T: Display> (bool, T) -> string = (cond, val) -> {
    if cond => val.show
    else    => ""
}
```

### Implicit Returns

The final expression in a function body is the return value.

```rs
let add := (x, y) -> x + y

let add := (x, y) -> {
    x + y
}
```

### `return` and `be`

`be` is a syntactic alternative to `return`. They are semantically identical and both
require a semicolon.

```rs
let add := (x, y) -> {
    be x + y;
}

let add := (x, y) -> {
    return x + y;
}
```

Implicit returns remain the idiomatic form. `be` exists for readability in places where
using `return` inside a constant-like binding reads poorly.

The exact scope where `be` is stylistically preferred is open. See [[open-questions]].

### `it` - Implicit Single-Argument Name

`it` is available only when the argument type is known from the annotation. Without an
annotation, an explicit argument name with `->` is required.

```rs
let double: (int) -> int       = { it * 2 }
let greet:  (string) -> string = { "Hello, " <> it }
let negate: (bool) -> bool     = { !it }

// Invalid:
let f := it * 2

// Valid:
let f := x -> x * 2
```

### `~` - Unnamed Lambda Arguments

`~` in an argument list, left of `->`, denotes one unnamed argument that flows implicitly
as the input to the right-side composition. At most one `~` may appear in an argument
list.

```rs
let map = ~, fn -> for_each(fn) >> collect
let negate_all = ~ -> map(negate)
```

`~` in expression position remains bitwise NOT. The parser distinguishes by context.

```rs
let mask := ~flags
```

### Named and Anonymous Function Types

Named function type aliases use `def` with `::`.

```rs
def BinaryIntOp  :: def((int, int) -> int)
def Predicate<T> :: def((T) -> bool)
def Transformer  :: def((string) -> string)
```

Anonymous function types in type positions use `def(...)` with the signature wrapped in a
second set of parentheses.

```rs
let apply: (int, def((int) -> int)) -> int = (n, f) -> { f(n) }
let map: <T> (Arr<T>, def((T) -> T)) -> Arr<T> = { ... }
```
