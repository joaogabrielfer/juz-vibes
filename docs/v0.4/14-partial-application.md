## 14. Partial Application

Using `_` in a regular function call, outside pipe placeholder context, creates a new
closure waiting for the missing argument or arguments. The resulting closure matches the
arity of the holes.

```rs
let add_one   := add(1, _)
let scale_by  := multiply(_, 10)
let partial   := f(_, "fixed", _)
let divide_by := flip(divide)(2.0, _)

arr |> map(add_one)
arr |> filter(greater(0, _))
```

Inside `|~`, `_` is not partial application. It is a pipe placeholder and must match the
number of piped values exactly.
