## 11. Pattern Matching & Control Flow

### `match` Expression

`match` consumes a specific value and dispatches on its shape. `match x []` is distinct
from `[]` alone, which defines a reusable function.

```rs
match result [
    .Ok(v)  => process(v)
    .Err(e) => panic e
]

match shape [
    .Circle { r }    => r * r * 3.14
    .Rect   { w, h } => w * h
    .Point           => 0.0
]

match status [
    .Active             => "running"
    .Inactive           => "stopped"
    (s) when s.is_error => "error"
]
```

For `expandable sum` root types, pattern matches must include `_`. The root is open, so
the compiler cannot prove exhaustive closure from currently visible members alone.

### `if` / `else`

```rs
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

### `be` and `return`

`be` and `return` are semantically identical. Both return from the current function or
lambda body, and both require a semicolon.

```rs
let calculate: (int) -> int = a -> {
    if a == 0 {
        be 0;
    };

    return a * 10;
}
```

`be` is not a block-yield mechanism. The v0.3 use of `be` to yield from inner block
expressions or list comprehensions is retired. The replacement yield syntax for list
comprehensions is still open.

### Destructuring in `let` Bindings

```rs
let a, b := pair_fn()

assert let a, b := pair_fn()

let .Ok(value) = result_fn(10)

assert let .Ok(value) = result_fn(10)

let .Some(.Ok(inner)) = nested_option_result
```

Destructuring without `assert` silently does nothing on mismatch. `assert let` crashes on
mismatch, with compile-time rejection when the mismatch is statically provable.

### `for` Loops

```rs
for x in collection {
    process(x);
}
```

List comprehension syntax exists as a design area, but its element-yield form must be
revisited because `be` no longer yields from inner block expressions. See
[[open-questions]].
