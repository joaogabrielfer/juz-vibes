## 14. Partial Application

Using `_` in a regular function call (outside pipe context) creates a new closure waiting
for the missing argument(s). The resulting closure matches the arity of the holes:

```rs
let add_one   := add(1, _);             // def (int) -> int
let scale_by  := multiply(_, 10);       // def (int) -> int
let partial   := f(_, "fixed", _);      // def (A, C) -> D
let divide_by := flip(divide)(2.0, _);  // def (float) -> float

// Used in pipelines:
arr |> map(add_one);          // [2, 3, 4, ...]
arr |> filter(greater(0, _)); // positive elements
```

---

