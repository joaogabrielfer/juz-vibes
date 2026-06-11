# Patterns and Control Flow

This chapter owns pattern dispatch, `match`, branches, destructuring control flow, loops,
and recursion inside pattern functions.

## Pattern-Matched Functions

Square brackets define a first-class function whose dispatch is a list of pattern arms.
This is the language's overload model for value-level elements. The compiler resolves
calls by the declared or inferred function type, then dispatches through the pattern arms
in source order. The compiler enforces exhaustiveness at compile time.

In a pattern matched function, the person can annotate the type without bindings, to, instead, bind them at the pattern level. As well as they can leave the type as inferred, or fully declare the type while binding at the patterns. Pattern matched functions can have many possible arg types, written up as `(int | string)` , when pre declaring the type, either at a foward declaration, or in the type section of a normal declaration. Or the person can declare the many types at each pattern in a  type inferred element.

```rust
let fib: (int) -> int = [
    0 => 0,
    1 => 1,
    n => recurse(n - 1) + recurse(n - 2),
]
```

`[]` blocks are values. They can be assigned, passed, piped, and stored.

Single-argument pattern functions should omit unnecessary parentheses. Multi-argument
pattern functions must group each arm's argument patterns with parentheses.

```rust
let choose: (bool, int, int) -> int = [
    (true,  a: int, _b: int) => a,
    (false, _a: int, b: int) => b,
]
```

## Guards

`when` attaches a boolean condition to a pattern arm. Guards are evaluated only after the
structural pattern matches. Guarded typed, variant, tuple, or otherwise complex patterns
should be parenthesized for readability.

```rust
let classify: int -> string = [
    (n: int) when n < 0  => "negative",
    (n: int) when n == 0 => "zero",
    (n: int)             => "positive",
]
```

## Range Patterns

```rust
let describe: int -> string = [
    0        => "zero",
    1..<10   => "single digit",
    10..<100 => "double digit",
    n: int   => "large",
]
```

## Union Inference

When a `[]` function has no explicit type declaration, its input union may be inferred
from arm patterns. All arms must agree on return type.

```rust
let stringify = [
    n: int      => Display::show(n),
    f: float    => Display::show(f),
    s: string   => s,
]
```

The inferred type is:

```rust
(int | float | string) -> string
```

## `recurse`

`recurse` calls the enclosing pattern function with new arguments.

```rust
let sum: int, int -> int = [
    (0, acc: int)      => acc,
    (n: int, acc: int) => recurse(n - 1, acc + n),
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
    .Ok(v)  => process(v),
    .Err(e) => panic e,
]

match shape [
    .Circle { r }    => r * r * 3.14,
    .Rect   { w, h } => w * h,
    .Point           => 0.0,
]
```

For `expandable sum` roots, pattern matches must include `_`.

Pattern guards in `match` follow the same style as pattern functions:

```rust
match result [
    .Ok(value) => value,
    (.Err(e)) when recoverable(e) => repair_error(e),
    .Err(e) => panic e,
]
```

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
let calculate :: int -> int;
let calculate = a -> {
    if a == 0 {
        return 0;
    };

    return a * 10;
}
```

Generator and collection-yield behavior uses the `Yield<T>` effect through `yield`,
`stream`, and `collect`. See [[effects-and-handlers]] and [[std-effects]].

## Destructuring Binds

Plain `let` accepts only irrefutable patterns. A plain destructuring bind is valid only
when the compiler can prove that the right-hand value has the required shape.

```rust
let a, b := pair_fn()
```

If the mismatch is statically provable, the compiler rejects the declaration.

```rust
// Invalid:
let a, b, c := pair_fn()
```

Refutable patterns belong in `if let` or `assert let`.

```rust
if let .Ok(value) := result_fn(10) {
    process(value)
}

assert let .Ok(value) := result_fn(10)
assert let .Some(.Ok(inner)) := nested_option_result
```

`if let` evaluates its block only when the pattern matches. Bindings introduced by
`if let` are scoped to the success block.

```rust
if let .Ok(value) := parse(input) {
    echo value
}

// Invalid:
echo value
```

`assert let` traps on runtime mismatch. The compiler rejects an `assert let` when the
mismatch is statically provable.

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
