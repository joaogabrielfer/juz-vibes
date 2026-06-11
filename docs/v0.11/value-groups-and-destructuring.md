# Value Groups and Destructuring

`(a, b)` denotes a value group: a language-level syntactic grouping of multiple values.
Value groups are not heap-allocated objects. They map directly to consecutive SLUR stack
slots.

```rust
let g := (1, 2)
let a, b := (1, 2)
let a, b, c := (1, 2)      // invalid: pattern does not match
assert let a, b := (1, 2)  // valid
```

Plain `let` accepts only irrefutable destructuring patterns. Use `if let` for
conditional destructuring and `assert let` when mismatch is a runtime failure.

## Multiple Assignment

```rust
let a, b := 1, 10
let a: int, b: int = 1, 10
```

## Value Group Types

In type signatures, `int, int -> int` means "takes two `int` values, returns `int`."
Parentheses are reserved for nested function types, grouped union types, and inline
named parameter lists.

```rust
let add_pair: int, int -> int = a, b -> a + b
```

## Anonymous Product Types

Use an inline product type when stored subelements need names.

```rust
let p: prod { x: int, y: int } = 10, 20
p.x
p.y

let distance:
    prod { x: int, y: int }, prod { x: int, y: int } -> float =
    a, b -> {
        let dx := a.x - b.x;
        let dy := a.y - b.y;
        sqrt(dx * dx + dy * dy)
    }
```

## Spread Operator

Spread is `..` for both value spreading and variadics.

```rust
let coords := (10, 20)

coords   |> draw_at(style)    // draw_at(coords, style)
coords.. |> draw_at(style)    // draw_at(10, 20, style)
```

Spread can pass a value group into a variadic position.

```rust
let msgs := ("a", "b", "c")
log("info", ..msgs)
```

Variadic parameters also use `..` and must be last.

```rust
let log: (level: string, args: ..string) -> void = { ... }
```
