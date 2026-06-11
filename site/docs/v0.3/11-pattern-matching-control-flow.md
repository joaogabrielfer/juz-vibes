## 11. Pattern Matching & Control Flow

### `match` Expression

Consumes a specific value and dispatches on its shape. `match x []` is distinct from `[]`
alone, which defines a reusable function:

```rs
match result [
    .Ok(v)  => process(v)
    .Err(e) => panic e
];

match shape [
    .Circle { r }    => r * r * 3.14
    .Rect   { w, h } => w * h
    .Point           => 0.0
];

match status [
    .Active             => "running"
    .Inactive           => "stopped"
    (s) when s.is_error => "error"
];
```

### `if`/`else`

```rs
// Block form:
let result := if condition {
    compute_something();
    final_value
} else {
    other_value
};

// One-liner with =>:
if x > 0    => handle_positive(x);
else if x < 0 => handle_negative(x);
else          => handle_zero();

// As expression:
let label := if x > 0 => "positive"
             else      => "non-positive";
```

### `be` and `return`

- **`be`** — yields a value from an **inner block expression**, assigning to the
  enclosing binding. Does not exit the parent function.
- **`return`** — exits the **parent function** immediately with a value.

```rs
let calculate: (int) -> int = a -> {
    let multiplier: int = {
        let @const DEFAULT: int = 10;
        if a > 5 { be 20 }     // yields 20 from inner block to 'multiplier'
        be DEFAULT              // yields 10 from inner block to 'multiplier'
    };
    return a * multiplier       // exits 'calculate'
};
```

### Destructuring in `let` Bindings

```rs
// Silent no-op on mismatch:
let a, b := pair_fn();

// Crash on mismatch:
assert let a, b := pair_fn();

// Result pattern — assigns only if Ok:
let .Ok(value) = result_fn(10);

// Result pattern — crash if Err:
assert let .Ok(value) = result_fn(10);

// Nested destructuring:
let .Some(.Ok(inner)) = nested_option_result;
```

### `for` Loops and List Comprehensions

```rs
// Imperative loop:
for x in collection {
    process(x);
};

// List comprehension — build array with be:
let evens: Arr<int> = [
    for x in arr {
        if x % 2 != 0 { continue };
        be x * 2
    }
];

// Nested comprehension:
let pairs: Arr<(int, int)> = [
    for i in 0..<n {
        for j in 0..<n {
            be (i, j)
        }
    }
];
```

---

