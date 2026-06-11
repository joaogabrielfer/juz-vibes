# Patterns and Control Flow

This chapter owns pattern dispatch, `match`, branches, destructuring control flow, loops,
and recursion inside pattern functions.

## Pattern-Matched Functions

Square brackets define a first-class function whose dispatch is a list of pattern arms.
This is the language's overload model for value-level elements. The compiler resolves
calls by the declared or inferred function type, then dispatches through the pattern arms
in source order. The compiler enforces exhaustiveness at compile time.

```rust
let fib: int -> int = [
    0 => 0,
    1 => 1,
    n => recurse(n - 1) + recurse(n - 2),
]
```

`[]` blocks are values. They can be assigned, passed, piped, and stored.

Single-argument pattern functions should omit unnecessary parentheses. Multi-argument
pattern functions must group each arm's argument patterns with parentheses.

```rust
let choose: bool, int, int -> int = [
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
let stringify := [
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
    .Err(e) when recoverable(e) => repair_error(e),
    .Err(e) => panic e,
]
```

## Core Pattern Forms

Pattern arms are tested in source order. The compiler may optimize dispatch only when the
observable arm order, guard order, binding behavior, and diagnostics are preserved.

### Wildcards and Binders

`_` matches any value and does not introduce a binding. A bare identifier introduces a
new binding.

```rust
match value [
    _ => "anything",
]
```

Typed binders narrow the matched value when the scrutinee type supports that narrowing.
For union inputs, a typed binder selects the compatible union member.

```rust
let describe: (int | string | bool) -> string = [
    x: int    => "integer",
    x: string => "string",
    x: bool   => "boolean",
]
```

For generic union inputs, union members must be provably disjoint for every valid generic
instantiation. A type error is produced when two members can unify under substitution.

```rust
// Valid if `Foo<T>` can never unify with `string`.
let handle: <T> (Foo<T> | string) -> unit = [
    .Foo(value) => use(value),
    s: string   => use_string(s),
]

// Invalid: `T` can be instantiated as `Fallible<X>`.
let bad: <T> (Fallible<T> | T) -> unit = [
    .Ok(value) => use(value),
    value: T   => use(value),
]
```

### Literal, Range, and Pinned Patterns

Literal patterns match by value. Integer, boolean, character, and string literal patterns
use the language's exact equality for those primitive values.

Float literal patterns are not part of the initial pattern surface because `NaN`, signed
zero, and exactness rules make equality diagnostics fragile. Use ranges or guards for
float classification.

```rust
match code [
    200 => "ok",
    400..<500 => "client error",
    _ => "other",
]
```

A bare identifier always binds. To compare against an existing in-scope value, use a
pinned value pattern.

```rust
let expected := "com"

match parts [
    [site, ^expected] => site,
    [site, ext]       => ext,
]
```

Pinned patterns require the pinned name to be in scope. Primitive pinned patterns use
built-in equality. User-defined pinned patterns require an appropriate equality surface,
such as `Eq<T>`, once that standard trait contract is finalized.

### Pattern Alternatives

`|` separates alternative patterns. It has low precedence in pattern position.

```rust
match n [
    0..<10 | 20 => "small or twenty",
    _ => "other",
]
```

An alternative pattern may bind names only when every alternative binds the same set of
names with equivalent types.

```rust
match value [
    .Left(x) | .Right(x) => use(x),

    // Invalid: only one alternative binds `x`.
    // .Left(x) | .Empty => use(x),
]
```

When a `when` guard follows an alternative, it applies to the whole alternative unless
parentheses create a different grouping.

### Alias Patterns

`pattern as name` matches `pattern` and also binds the whole matched value as `name`.
The alias has the original scrutinee type for that pattern position.

```rust
match xs [
    [head, ..tail] as whole => use(whole, head, tail),
]
```

Alias bindings follow the same alternative-binding rule as ordinary binders.

```rust
match value [
    (.A(x) as whole) | (.B(x) as whole) => use(whole, x),
]
```

Inside nested extracted or transformed pattern contexts, an alias binds the value being
matched at that nested position, not any outer scrutinee.

### Sum and Product Patterns

Variant patterns use the leading-dot variant surface when the scrutinee type determines
the sum root.

```rust
match result [
    .Ok(value) => value,
    .Err(e)   => panic e,
]
```

When the scrutinee type is ambiguous, the pattern must use a type-qualified variant name.
Typed binders must also be explicit when ordinary binder inference would be ambiguous.

```rust
match value [
    Fallible.Ok(value) => use(value),
    e: Error           => report(e),
]
```

Product-like variant and product patterns use Rust-like field patterns. Field shorthand
binds a local with the same name as the field. `field: name` binds the field to a
different local name. `..` ignores remaining visible fields.

```rust
match shape [
    .Circle { r } => r * r * 3.14,
    .Rect { w, h } => w * h,
    .Rect { w: width, h: height } => width * height,
]

match user [
    User { id, .. } => fetch(id),
]
```

Product and field patterns respect module visibility. A module cannot match private
subelements of a type declared elsewhere unless that type exposes a public pattern
surface in a later design.

Tuple-like variant payloads are matched positionally.

```rust
match shape [
    .Square(size) => size * size,
    .Rectangle(w, h) => w * h,
]
```

### Sequence Patterns

Sequence patterns use square brackets. Without a rest collector, the pattern matches an
exact number of elements.

```rust
match parts [
    [] => "empty",
    [site, "com"] => "{site} is a .com site",
    [site, owner, "com"] => "{site} is part of {owner}",
]
```

`..name` is a pattern-only rest collector. It binds the remaining elements as the same
sequence family as the scrutinee when possible, initially `Arr<T>`.

```rust
let add_1: (Arr<int>) -> Arr<int> = [
    [] => [],
    [head, ..tail] => [head + 1, add_1(tail)..],
]
```

The rest collector may appear at the beginning, middle, or end of a sequence pattern. At
most one rest collector is allowed per sequence pattern.

```rust
match parts [
    [head, ..tail] => use(head, tail),
    [..prefix, last] => use(prefix, last),
    [first, ..middle, last] => use(first, middle, last),
]
```

The minimum length is the number of non-rest element patterns. `[head, ..tail]` and
`[..prefix, last]` require at least one element. `[first, ..middle, last]` requires at
least two elements.

Sequence patterns are refutable unless the compiler can prove the required shape. Plain
`let` therefore rejects ordinary dynamic-array head/tail patterns, while `if let` and
`assert let` may use them.

```rust
// Invalid for ordinary `Arr<T>`:
// let [head, ..tail] := values

if let [head, ..tail] := values {
    use(head, tail)
}

assert let [head, ..tail] := values
```

Inside a successful sequence-pattern arm, the matched elements are proven present. This
acts like a local index proof for the elements introduced by the pattern.

### Regex Patterns

`re!` is a literal macro that produces a compile-time checked `Regex` value. In pattern
position, a `Regex` performs a full match against a `string` scrutinee. Search behavior
belongs in ordinary APIs such as a future `Regex::search`.

The `re!` literal receives uninterpreted literal contents. Ordinary string escaping is
not applied before the regex macro handles the text.

```rust
match get_file_name() [
    re!".*\.exe" => println("its an executable"),
    re!".*\.juz" => println("its source code"),
]
```

Capture binding is explicit and positional.

```rust
match get_file_name() [
    re!("main\.(.*)", ext) => println("the extension of main is {ext}"),
    _ => println("not main"),
]
```

The number of binders must match the number of capture groups that are guaranteed to
participate in every successful match. Regex patterns with optional, repeated, or
branch-dependent captures that may fail to produce a binder are rejected in binding form
for this revision.

Named captures do not bind automatically. Regex capture binders follow the same
alternative-binding rules as other pattern binders.

```rust
match text [
    re!("a(.*)", x) | re!("b(.*)", x) => use(x),

    // Invalid: `x` is not bound by every alternative.
    // re!("a(.*)", x) | re!"b" => use(x),
]
```

### Redundancy and Exhaustiveness

Matches and pattern functions are checked for exhaustiveness when the compiler has enough
static information about the scrutinee type. Sum types, enums, literal booleans, finite
integer ranges, fixed arrays, and structural sequence length constraints may contribute
to checking.

Guards and regex patterns are runtime filters. They do not make a match exhaustive unless
the compiler can prove the guard is always true or the regex analysis belongs to a later
explicitly supported finite subset.

```rust
// Invalid: the guard does not cover every `int`.
let only_positive: int -> string = [
    n: int when n > 0 => "positive",
]
```

Obvious structural redundancy is a compile error. Redundancy involving guards, regexes,
or future extractor patterns may be reported as a warning when the compiler cannot prove
the case unreachable with certainty.

```rust
match xs [
    [..all] => all,

    // Invalid: unreachable after `[..all]`.
    // [a, b] => [a, b],
]
```

Repeated binders in one pattern are invalid. Use a guard for equality constraints.

```rust
// Invalid:
// [x, x] => x

[x, y] when x == y => x
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

## Index Proofs

`[]` indexing is accepted only when the compiler can prove it is safe for the specific
value being indexed.

For fixed arrays, obvious literal proofs are accepted directly.

```rust
let a :: int[3]

let first := a[0]
// Invalid:
// let bad := a[3]
```

For dynamic arrays, `has_index` refines the checked index inside the success branch.

```rust
if values.has_index(i) {
    let value := values[i]
}
```

Iterating `arr.indices` introduces a loop binder that is treated as valid for that
specific array value during the loop body.

```rust
for i in values.indices {
    echo values[i]
}
```

Index proofs are flow-sensitive. Mutations that may change array length, occupancy, or
index mapping invalidate previously established proofs.

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
