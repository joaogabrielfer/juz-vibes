## 10. Pattern-Matched Functions — [] Blocks

Square brackets define a **function whose dispatch is a list of pattern arms**. The
compiler enforces exhaustiveness at compile time. `[]` blocks are first-class values —
they can be assigned, passed, piped, and stored:

```rs
let fib: (int) -> int = [
    (0) => 0
    (1) => 1
    (n: int) => recurse(n - 1) + recurse(n - 2)
];
```

### `when` Guard Clauses

Attach arbitrary boolean conditions to pattern arms. Guards are evaluated only if the
structural pattern matches:

```rs
let classify: (int) -> string = [
    (n: int) when n < 0  => "negative"
    (n: int) when n == 0 => "zero"
    (n: int)             => "positive"
];

let grade: (int) -> string = [
    (n: int) when n >= 90 => "A"
    (n: int) when n >= 80 => "B"
    (n: int) when n >= 70 => "C"
    (n: int) when n >= 60 => "D"
    (n: int)              => "F"
];
```

### Range Patterns

```rs
let describe: (int) -> string = [
    (0)        => "zero"
    (1..<10)   => "single digit"
    (10..<100) => "double digit"
    (n: int)   => "large"
];
```

### `recurse` — Anonymous Self-Reference

`recurse` inside a `[]` block calls the enclosing pattern function with new arguments.
The compiler applies tail-call optimization (TCO) when `recurse` is in tail position.
When it is not, the compiler warns and attempts auto-TCO:

```rs
// Manual tail-recursive (explicit accumulator — TCO applied):
let sum: (int, int) -> int = [
    (0, acc: int)      => acc
    (n: int, acc: int) => recurse(n - 1, acc + n)
];

// Not tail-recursive — auto-TCO attempted:
let factorial: (int) -> int = [
    (0) => 1
    (n: int) => n * recurse(n - 1)   // compiler introduces accumulator silently
];
```

### Auto-TCO

When `recurse` is separated from tail position by a single associative operation, the
compiler silently introduces an accumulator and rewrites to tail-recursive form. This
applies to `*`, `+`, `<>`, and other `Semigroup`-satisfying operations:

**Compiler behavior:**

| Situation | Compiler action |
|---|---|
| `recurse` in tail position | TCO applied silently |
| `recurse` separated by single associative op | Auto-TCO — accumulator introduced silently |
| `recurse` in non-optimizable position | Warning emitted |
| `#[tco]` attribute present, TCO provable | TCO forced and verified |
| `#[tco]` attribute present, TCO unprovable | **Compile error** |

---

