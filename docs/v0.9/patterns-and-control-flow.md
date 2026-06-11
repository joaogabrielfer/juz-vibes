# Patterns and Control Flow

This chapter owns pattern dispatch, `match`, branches, destructuring control flow, loops,
and recursion inside pattern functions.

## Pattern-Matched Functions

Square brackets define a first-class function whose dispatch is a list of pattern arms.
The compiler enforces exhaustiveness at compile time.

```rust
let fib: (int) -> int = [
    (0) => 0
    (1) => 1
    (n: int) => recurse(n - 1) + recurse(n - 2)
]
```

`[]` blocks are values. They can be assigned, passed, piped, and stored.

## Guards

`when` attaches a boolean condition to a pattern arm. Guards are evaluated only after the
structural pattern matches.

```rust
let classify: (int) -> string = [
    (n: int) when n < 0  => "negative"
    (n: int) when n == 0 => "zero"
    (n: int)             => "positive"
]
```

## Range Patterns

```rust
let describe: (int) -> string = [
    (0)        => "zero"
    (1..<10)   => "single digit"
    (10..<100) => "double digit"
    (n: int)   => "large"
]
```

## Union Inference

When a `[]` function has no explicit type declaration, its input union may be inferred
from arm patterns. All arms must agree on return type.

```rust
let stringify = [
    (n: int)    => Display.show(n)
    (f: float)  => Display.show(f)
    (s: string) => s
]
```

The inferred type is:

```rust
(int | float | string) -> string
```

## `recurse`

`recurse` calls the enclosing pattern function with new arguments.

```rust
let sum: (int, int) -> int = [
    (0, acc: int)      => acc
    (n: int, acc: int) => recurse(n - 1, acc + n)
]
```

The compiler applies tail-call optimization when `recurse` is in tail position. When
`recurse` is separated from tail position by a single associative operation, the compiler
may introduce an accumulator and rewrite to tail-recursive form.

| Situation | Compiler action |
|---|---|
| `recurse` in tail position | TCO applied silently |
| `recurse` separated by single associative op | Auto-TCO; accumulator introduced silently |
| `recurse` in non-optimizable position | Warning emitted |
| `@tco` attribute present, TCO provable | TCO forced and verified |
| `@tco` attribute present, TCO unprovable | Compile error |

## `match`

`match` consumes one value and dispatches on its shape. `match x []` is distinct from
`[]`, which defines a reusable function.

```rust
match result [
    .Ok(v)  => process(v)
    .Err(e) => panic e
]

match shape [
    .Circle { r }    => r * r * 3.14
    .Rect   { w, h } => w * h
    .Point           => 0.0
]
```

For `expandable sum` roots, pattern matches must include `_`.

## `if` and `else`

`if` is expression-oriented.

```rust
let result := if condition {
    compute_something();
    final_value
} else {
    other_value
}

if x > 0      => handle_positive(x)
else if x < 0 => handle_negative(x)
else          => handle_zero()

let label := if x > 0 => "positive"
             else      => "non-positive"
```

## `be` and `return`

`be` and `return` return from the current function or lambda body. They do not yield from
inner blocks.

```rust
let calculate: (int) -> int = a -> {
    if a == 0 {
        be 0;
    };

    return a * 10;
}
```

Generator and collection-yield behavior uses the `Yield<T>` effect through `yield`,
`stream`, and `collect`. See [[effects-and-handlers]] and [[std-effects]].

## Destructuring Binds

```rust
let a, b := pair_fn()

assert let a, b := pair_fn()

let .Ok(value) = result_fn(10)

assert let .Ok(value) = result_fn(10)

let .Some(.Ok(inner)) = nested_option_result
```

Destructuring without `assert` silently does nothing on mismatch. `assert let` crashes on
mismatch, with compile-time rejection when the mismatch is statically provable.

## `for`

```rust
for x in collection {
    process(x);
}
```

The old list-comprehension syntax is retired. Use `collect` for eager collections and
`stream` for lazy streams.

```rust
let values := collect<Arr> {
    for x in collection {
        if keep(x) {
            yield transform(x)
        }
    }
}
```
