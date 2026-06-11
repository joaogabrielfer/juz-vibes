## 10. Pattern-Matched Functions - [] Blocks

Square brackets define a **function whose dispatch is a list of pattern arms**. The
compiler enforces exhaustiveness at compile time. `[]` blocks are first-class values: they
can be assigned, passed, piped, and stored.

```rs
let fib: (int) -> int = [
    (0) => 0
    (1) => 1
    (n: int) => recurse(n - 1) + recurse(n - 2)
]
```

### `when` Guard Clauses

Attach arbitrary boolean conditions to pattern arms. Guards are evaluated only if the
structural pattern matches.

```rs
let classify: (int) -> string = [
    (n: int) when n < 0  => "negative"
    (n: int) when n == 0 => "zero"
    (n: int)             => "positive"
]
```

### Range Patterns

```rs
let describe: (int) -> string = [
    (0)        => "zero"
    (1..<10)   => "single digit"
    (10..<100) => "double digit"
    (n: int)   => "large"
]
```

### Union Inference from Arms

When a `[]` function has no explicit type declaration, its input union may be inferred
from arm patterns. All arms must agree on return type.

```rs
let stringify = [
    (n: int)    => n.show
    (f: float)  => f.show
    (s: string) => s
]
```

The inferred type is:

```rs
(int | float | string) -> string
```

### `recurse` - Anonymous Self-Reference

`recurse` inside a `[]` block calls the enclosing pattern function with new arguments. The
compiler applies tail-call optimization when `recurse` is in tail position. When it is
not, the compiler warns and attempts auto-TCO.

```rs
let sum: (int, int) -> int = [
    (0, acc: int)      => acc
    (n: int, acc: int) => recurse(n - 1, acc + n)
]

let factorial: (int) -> int = [
    (0) => 1
    (n: int) => n * recurse(n - 1)
]
```

### Auto-TCO

When `recurse` is separated from tail position by a single associative operation, the
compiler silently introduces an accumulator and rewrites to tail-recursive form. This
applies to `*`, `+`, `<>`, and other `Semigroup`-satisfying operations.

| Situation | Compiler action |
|---|---|
| `recurse` in tail position | TCO applied silently |
| `recurse` separated by single associative op | Auto-TCO; accumulator introduced silently |
| `recurse` in non-optimizable position | Warning emitted |
| `@tco` attribute present, TCO provable | TCO forced and verified |
| `@tco` attribute present, TCO unprovable | Compile error |
