## 9. Function Syntax & Lambdas

### Universal Function Form

```
let [mut] [attrs] name: <generics> (args) -> return_type = body;
```

### Syntax Gradient

```rs
// Fully inferred, named argument:
let double := x -> x * 2;

// Fully inferred, multi-argument:
let add := (x, y) -> x + y;

// Typed, implicit `it` (single-argument shorthand):
let double: (int) -> int = { it * 2 };

// Typed, named argument in body:
let square: (int) -> int = { it * it };

// Named arguments in signature:
let offset: (base: int, scale: int) -> int = { base + scale * 16 };

// Full body with statements:
let process: (int) -> int = x -> {
    let doubled  := x * 2;
    let adjusted := doubled + 16;
    adjusted
};

// Generic:
let identity: <T> (T) -> T = { it };

// Bounded generic:
let show_if: <T: Display> (bool, T) -> string = (cond, val) -> {
    if cond => val.show
    else    => ""
};

// Generic with multiple bounds:
let log_and_compare: <T: Display + Ord<T>> (T, T) -> string = (a, b) -> {
    let result := a.compare(b);
    format("{} vs {}: {}", a.show, b.show, result)
};
```

### `it` — Implicit Single-Argument Name

Available **only** when the argument type is known from the annotation. Without an
annotation, an explicit argument name with `->` is required:

```rs
let double: (int) -> int       = { it * 2 };    // valid
let greet:  (string) -> string = { "Hello, " <> it };   // valid
let negate: (bool) -> bool     = { !it };        // valid

let f := it * 2;   // COMPILE ERROR — no annotation, cannot infer type of it
                   // correct: let f := x -> x * 2;
```

### Named and Anonymous Function Types

```rs
// Named type alias:
def BinaryIntOp  = def (int, int) -> int;
def Predicate<T> = def (T) -> bool;
def Transformer  = def (string) -> string;

// Inline anonymous function type in argument position:
let apply:  (int, def (int) -> int) -> int = (n, f) -> { f(n) };
let filter: <T> (Arr<T>, def (T) -> bool) -> Arr<T> = { ... };
```

---

