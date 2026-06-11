## 5. Value Groups & Destructuring

`(a, b)` denotes a **value group** — a language-level syntactic grouping of multiple
values. Value groups are not heap-allocated objects. They map directly to consecutive stack
slots on the SLUR VM, making them zero-cost at runtime.

```rs
let g := (1, 2);
let a, b := (1, 2);         // destructuring bind — both succeed
let a, b, c := (1, 2);      // silent no-op — pattern does not match
assert let a, b := (1, 2);  // crash on mismatch (compile-time where possible)
```

### Multiple Assignment

```rs
let a, b := 1, 10;               // inferred types
let a: int, b: int = 1, 10;      // explicit types
```

### Anonymous Struct Types

Prefixing with `def` in a type annotation creates an anonymous named struct inline:

```rs
let p: def (x: int, y: int) = 10, 20;
p.x;   // 10
p.y;   // 20

// In function argument position:
let distance: (def (x: int, y: int), def (x: int, y: int)) -> float = (a, b) -> {
    let dx := a.x - b.x;
    let dy := a.y - b.y;
    sqrt(dx*dx + dy*dy)
};
```

### Spread Operator `...`

Unpacks a value group into its constituent values at the call site:

```rs
let t := (1, 2);

t     |> f(3, 4);    // f(t, 3, 4)     — group passed as a single value
t...  |> f(3, 4);    // f(1, 2, 3, 4)  — group unpacked into positional args
```

In type signatures, `(int, int) -> int` means "takes a group of two `int`s, returns `int`."

---

