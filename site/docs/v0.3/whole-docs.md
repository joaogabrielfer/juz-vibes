# Element Language Specification
### Revision 0.3 — Working Draft

---

## Table of Contents

- [[#1. Core Philosophy & Architecture]]
- [[#2. Syntax Fundamentals]]
- [[#3. Declarations, Bindings & Attributes]]
- [[#4. Thunk Semantics & Lazy Evaluation]]
- [[#5. Value Groups & Destructuring]]
- [[#6. Type System]]
- [[#7. Traits, extend & Implementations]]
- [[#8. The Type Primitive & Reflection]]
- [[#9. Function Syntax & Lambdas]]
- [[#10. Pattern-Matched Functions — [] Blocks]]
- [[#11. Pattern Matching & Control Flow]]
- [[#12. Generics]]
- [[#13. Pipe Operators & Data Flow]]
- [[#14. Partial Application]]
- [[#15. Operator System]]
- [[#16. Standard Combinators]]
- [[#17. Error Handling]]
- [[#18. The use Keyword — Callback Flattening]]
- [[#19. Dot Access Model]]
- [[#20. Metaprogramming Pipeline]]
- [[#21. Concurrency Model *(Outline — Full Spec Pending)*]]
- [[#22. Build System & Tooling]]
- [[#23. Low-Level Control]]
- [[#24. Open / Not Yet Specified]]

---

## 1. Core Philosophy & Architecture

Element is built on a single unifying principle: **Everything is an Element**. Variables,
functions, types, traits, and modules are structurally identical constructs. A variable is a
zero-argument function. A function is a named lambda. There is no categorical distinction
between data and behavior.

Three foundational rules govern all Element code:

- **Immutable by default** — all bindings are immutable unless explicitly marked `mut`
- **Private by default** — all elements are module-local unless marked `@pub`
- **Expression-oriented** — every block and construct evaluates to a value; the final
  expression in any block is its implicit return value

**Compilation target:** SLUR, a custom flat-stack virtual machine. The stack-based
architecture enables zero-cost abstractions for value groups, pattern matching, and error
propagation without heap allocation overhead. Because the VM operates on a flat data stack,
tuple destructuring, currying, and monadic chaining unroll directly into sequential
bytecode with no boxing.

---

## 2. Syntax Fundamentals

### Comments

```rs
// Single-line comment

/*
    Multi-line block comment.
    Can span any number of lines.
*/
```

### Identifiers

Standard alphanumeric identifiers with underscores. Conventions:

- `PascalCase` — types, type traits, and modules
- `snake_case` — values, functions, and local bindings
- `SCREAMING_SNAKE_CASE` — compile-time constants
- `(operator_symbol)` — operator functions, wrapped in parentheses: `(+)`, `(<$>)`, `(|>?)`

### Semicolons

Semicolons are **mandatory** at the end of all statements inside `{}` block scopes. The
single exception is the **final expression** in a block, which omits its semicolon and
acts as the block's implicit return value:

```rs
let compute: (int, int) -> int = (base, scale) -> {
    let raw := base * scale;     // statement — semicolon required
    let adjusted := raw + 16;    // statement — semicolon required
    adjusted                     // final expression — no semicolon, implicitly returned
};
```

At module (top-level) scope, semicolons terminate declarations.

### The Fat Arrow =>

=> is the **single-line expression body selector**. It is strictly banned from top-level
`let` declarations and is only valid inside expression branches:

- `match` arms
- `if`/`else` one-liners
- `[]` pattern matching arms
- `when` guard bodies

```rs
// BANNED — compile error:
let f: (int) -> int => it * 2;

// ALLOWED — interior branch:
if x > 0   => echo "positive";
else if x == 0 => echo "zero";
else           => echo "negative";
```

---

## 3. Declarations, Bindings & Attributes

### Universal Binding Form

Every binding in Element follows the same structural sequence:

```
let [mut] [inline-attributes] name: type = [use] value;
```

The type annotation is optional when using the inferred `:=` form.

### Assignment Operators

- `:=` — inferred binding; compiler determines the type from the right-hand side
- =  — explicit binding; type annotation on the left is required

```rs
let x: int = 10;
let x := 10;              // inferred — equivalent
let mut @pub counter: int = 0;
```

### `mut` Keyword

`mut` immediately follows `let` and marks a binding as mutable. It always precedes any
inline attributes:

```rs
let mut score: int = 0;
let mut @pub @lazy live_value: int = compute();
```

Attempting to reassign a non-`mut` binding is a compile error.

### Attribute System — Two Levels

Element's attribute system distinguishes between attributes that modify the **nature** of
a declaration (declaration-level) and those that annotate a binding (inline).

#### Declaration-Level Attributes

Placed on the line immediately above the binding. They signal that what follows is not a
standard element:

```rs
@macro
let json!: (TokenStream) -> TokenStream = { ... };

@operator
let (+): (MyInt, MyInt) -> MyInt = { ... };

@operator
let @position(postfix) @bind(left->a) @precedence(0)
(?): <T, E> (Result<T, E>) -> T = { return_if_err(it) };

@comptime
let validate_layout: (Type) -> void = t -> {
    if t.size > 128 => panic "Exceeds stack frame limit!";
};
```

#### Inline Attributes

Follow `let` (after `mut` if present) and annotate the binding itself:

| Attribute | Meaning |
|---|---|
| `@pub` | Export from module; visible to importers |
| `@const` | Compile-time constant; zero runtime allocation |
| `@inline` | Inline at every call site |
| `@lazy` | Remove thunk cache; re-evaluate on every bare access |
| `@impl(Trait)` | Bind as implementation of the named trait |
| `@impl(Trait, priority: N)` | Implementation with explicit override priority |
| `@extern("symbol")` | Link to external symbol by name |
| `@deprecated("msg")` | Emit a deprecation warning at usage sites |
| `@test` | Mark as a unit test; excluded from non-test builds |
| `@align(n)` | Memory alignment hint in bytes |
| `@packed` | No struct padding |
| `@memoize` | Cache result on first invocation (pure functions only) |
| `@auto_impl(...)` | Derive trait implementation automatically (see §7) |

Under `@operator`, additional inline attributes configure fixity:

| Attribute | Meaning |
|---|---|
| `@position(infix/prefix/postfix)` | Where the symbol sits relative to its arguments |
| `@bind(left->a, right->b)` | Map positional sources to parameter names |
| `@assoc(left/right)` | Chain grouping direction |
| `@precedence(n)` | Numeric precedence level |

#### Statement-Level Directives

`@` also serves as a statement-scope directive, primarily for compile-time operations:

```rs
@embed("shaders/frag.spv")          // embed file at this location
@run validate_config();              // execute at compile time, error if it fails
@insert "generated/routes.elem"     // textual file inclusion
@if TARGET == .wasm { ... }         // conditional compilation block
@assert sizeof(PacketHeader) == 32; // compile-time size assertion

// Right-side variants (embed/run on the value side):
let SHADER: Arr<u8>  = @embed("shaders/frag.spv");
let PRIMES: Arr<int> = @run generate_primes(1000);
```

### Constants

The `@const` attribute replaces the retired `::` syntax (formerly borrowed from JAI):

```rs
// Retired — no longer valid:
let MAX :: 100;

// Current form:
let @const MAX: int = 100;
let @pub @const VERSION: string = "1.0.0";
let @pub @const PI: float = 3.14159265358979;
```

---

## 4. Thunk Semantics & Lazy Evaluation

Every zero-argument binding is a **managed thunk**. Two evaluation modes govern how and
when the body is executed.

### Default — First-Access Caching

A zero-argument element evaluates its body exactly once, on the first bare access in its
scope. All subsequent bare accesses return the cached value. Appending `()` forces a fresh
re-evaluation, bypassing the cache:

```rs
let @pub timestamp: int = get_unix_time();

let t1 := timestamp;    // body evaluated here — result cached
let t2 := timestamp;    // cache hit — same value as t1
let t3 := timestamp();  // forces fresh evaluation — new unix timestamp
```

`foo` accesses the value. `foo()` calls the function again.

### `@lazy` — Always Re-Evaluate

The `@lazy` attribute removes the internal cache entirely. Every bare access triggers a
fresh execution of the body. For a `@lazy` element, `foo` and `foo()` are equivalent:

```rs
let @pub @lazy live_clock: int = get_unix_time();

let c1 := live_clock;    // evaluated dynamically
let c2 := live_clock;    // evaluated again — different result
let c3 := live_clock();  // same as bare access for @lazy
```

### Practical Application — Computed Properties

The thunk model makes computed properties ergonomic without method-call syntax. The `Time`
type demonstrates this clearly:

```rs
def @impl(Display) Time = extend {
    self: int;   // the one real value: unix timestamp in seconds

    ms:   int = { self * 1_000 };
    us:   int = { self * 1_000_000 };
    ns:   int = { self * 1_000_000_000 };
    days: int = { self / 86_400 };

    show: string = { format("{}s", self) };
};

let t := now();

echo t.ms;    // computed on first access, cached — reads like a field
echo t.days;  // same
echo t.ns;    // same
echo t.show;  // "1718000000s"
```

Each computed property is evaluated once on first access, then cached for the lifetime of
that binding. There are no parentheses, no method call semantics, no cognitive overhead.

---

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

## 6. Type System

### The `def` Keyword

`def` is the universal type declaration keyword, replacing the retired `type` keyword. It
covers all type forms: product types, sum types, enums, unions, aliases, type traits, and
`extend` declarations:

```rs
def Point    = prod { x: int, y: int };
def Shape    = sum  { Circle { r: float }, Rect { w: float, h: float } };
def Status   : enum<int> { Active = 1, Inactive, Pending }
def UserId   = int;
def Callback = def (int) -> string;
```

### Product Types (`prod {}`)

Named structs. All fields are always present. Runtime size is the sum of all field sizes
(plus any alignment padding):

```rs
def User = prod {
    id:    int,
    name:  string,
    email: string,
};

let mut u := User(1, "Alice", "alice@example.com");
u.name = "Bob";    // valid — u is mut
```

### Sum Types (`sum {}`)

Tagged unions. Exactly one variant is active at runtime. Size is the discriminant tag plus
the largest variant's size:

```rs
def Shape = sum {
    Circle { r: float },
    Rect   { w: float, h: float },
    Point,
};

def Result: <T, E> = sum {
    Ok(T),
    Err(E),
};

// Variant construction via dot-shorthand (type inferred from context):
let s: Shape = .Circle { r: 5.0 };
let r: Result<int, string> = .Ok(42);
let e: Result<int, string> = .Err("not found");
```

### C-Style Enums (`enum<T>`)

Integer-backed enumerations. Fields are implicitly sequential unless overridden. The backing
type `T` must be an integer primitive:

```rs
def Direction: enum<int> { North, South, East, West }

def Priority: enum<u8> {
    Low    = 1,
    Medium,    // 2
    High,      // 3
    Critical,  // 4
}
```

### Untagged Unions (`union`)

Unsafe raw memory overlap. All fields share the same memory. Size is the largest field.
Use only for low-level interop:

```rs
def RawValue: union {
    as_int:   int,
    as_float: float,
    as_bytes: Arr<u8>,
};
```

### Type Aliases

```rs
def UserId   = int;
def Matrix   = Arr<Arr<float>>;
def Handler  = def (Request) -> Response;
```

### Primitive Types

`int`, `float`, `bool`, `string`, `char`, `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`,
`i64`, `f32`, `f64`, `ptr`. Full numeric tower specification pending.

---

## 7. Traits, extend & Implementations

Element has two distinct categories of traits that serve fundamentally different purposes.

### Behavioral Traits

Define method signatures a type must implement. These are interface contracts — comparable
to Rust traits. Declared with `def Name = trait {}`. The special type `Self` refers to
the implementing type:

```rs
def Display = trait {
    show: (Self) -> string,
};

def Eq<T> = trait {
    equals: (T, T) -> bool,
};

def Ord<T> = trait {
    compare: (T, T) -> int,
};

def Hash = trait {
    hash: (Self) -> u64,
};
```

### Type Traits

Define structural/data fields a type must carry. These fields exist **only in the
compiler's static type registry** — they are completely erased from the output binary.
Type traits enable higher-kinded abstractions (Functor, Monad, Monoid, etc.):

```rs
def Monoid<T> = trait {
    empty:   T,
    combine: (T, T) -> T,
};

def Semigroup<T> = trait {
    combine: (T, T) -> T,
};

def Functor<F> = trait {
    map: <A, B> (def (A) -> B, F<A>) -> F<B>,
};

def Applicative<F> = trait {
    pure:  <A> (A) -> F<A>,
    apply: <A, B> (F<def (A) -> B>, F<A>) -> F<B>,
};

def Monad<M> = trait {
    bind: <A, B> (M<A>, def (A) -> M<B>) -> M<B>,
};

def Collection<C, Item> = trait {
    next: (C) -> Result<Item, string>,
};
```

### `extend` — Type Definition with Trait Implementations

`extend` declares a new type and simultaneously satisfies one or more traits. The `self`
block defines the actual runtime data layout. All other fields satisfy trait requirements
and are erased at compile time.

A variable of an `extend`-defined type takes up exactly the memory of its `self` block
at runtime — no boxing, no vtables, no overhead:

```rs
def @impl(Monad, Applicative, Functor) Option<T> = extend {
    self: sum {
        Some(T),
        None,
    };

    // Functor
    map = <A, B> (f, context) -> {
        match context [
            .Some(val) => .Some(f(val))
            .None      => .None
        ]
    };

    // Applicative
    pure  = val -> .Some(val);
    apply = <A, B> (wrapped_f, wrapped_val) -> {
        match wrapped_f [
            .Some(f) => f <$> wrapped_val
            .None    => .None
        ]
    };

    // Monad
    bind = <A, B> (context, f) -> {
        match context [
            .Some(val) => f(val)
            .None      => .None
        ]
    };
};
```

#### Field Definition Syntax Inside `extend`

Fields follow the same rules as `let` bindings but without the `let` keyword. Two valid
forms:

```rs
def @impl(Display) Point = extend {
    self: prod { x: int, y: int };

    // Short form — type inferred from trait definition, self is implicit:
    show = { format("({}, {})", self.x, self.y) };

    // Long form — explicit types and argument name:
    show: (s: Self) -> string = { format("({}, {})", s.x, s.y) };
};
```

In the short form, `self` is implicitly bound to the current instance. In the long form,
you explicitly name the argument (here `s`), and `self` is no longer implicit — use `s`
in the body instead.

### Implementing Behavioral Traits on Existing Types

A type defined with `prod`, `sum`, or other keywords can receive trait implementations
at any point, including types from other modules. The compiler forbids modifying the
original `self` layout:

```rs
// Defined in std — the original type:
def Option<T> = sum { Some(T), None };

// In your module — extend without self block:
// self is implicitly Option<T> since the type already exists
def @impl(Display) Option<T> = extend {
    show = {
        match self [
            .Some(v) => format("Some({})", v.show)
            .None    => "None"
        ]
    };
    // Adding a self: block here is a compile error —
    // you cannot change an existing type's layout
};

// Multiple traits at once:
def @impl(Display, Hash) Point = extend {
    show = { format("({}, {})", self.x, self.y) };
    hash = { hash_combine(self.x, self.y) };
};
```

### `@auto_impl` — Derived Implementations

A built-in macro that generates `extend` blocks for type traits with straightforward field
mappings. Equivalent to Rust's `#[derive(...)]` but user-extensible:

```rs
@auto_impl(Monoid(empty: 0, combine: (+)))
def Score = prod {
    value: int,
};

// Compiler generates:
// def @impl(Monoid) Score = extend {
//     self: prod { value: int };
//     empty   = 0;
//     combine = (+);
// };

// Multiple:
@auto_impl(Eq(equals: (==)), Ord(compare: int_compare))
def Priority: enum<int> { Low = 1, Medium, High }
```

### Trait Coherence & `override`

Each trait can be implemented once per type per module boundary. Conflicts require explicit
resolution using `override`:

```rs
// Two modules implement Display for ExternalType:
override(priority: 2) impl ExternalType for Display = module_a_impl;

// Rules:
// — One impl, no conflict                     → no annotation needed
// — Two impls, no override                    → compile error
// — Two overrides, different priorities       → higher priority wins
// — Two overrides, same priority              → compile error
```

---

## 8. The Type Primitive & Reflection

`Type` (capitalized) is a first-class compile-time value populated by the compiler. It
represents structural metadata about any type and is available as a parameter type or
value. `Type` does not conflict with the `def` keyword:

```rs
let size_of: (Type) -> int = t -> t.size;
let name_of: (Type) -> string = t -> t.name;

size_of(Point)    // sizeof(Point) in bytes
name_of(int)      // "int"
```

### Universal Fields on All `Type` Values

| Field | Return Type | Meaning |
|---|---|---|
| `.size` | `int` | Size in bytes |
| `.align` | `int` | Alignment requirement in bytes |
| `.name` | `string` | String name of the type |
| `.fields` | `Arr<FieldInfo>` | For `prod` types: names, types, and byte offsets of fields |
| `.variants` | `Arr<VariantInfo>` | For `sum`/`enum`: variant names and payload types |
| `.backing` | `Type` | For `enum<T>`: the integer backing type `T` |
| `.field_count` | `int` | Number of fields (for `prod`) or variants (for `sum`/`enum`) |

### Runtime Type Access on Values

Any value exposes a `.type` property that returns its `Type` at runtime:

```rs
let s: Shape = .Circle { r: 5.0 };

echo s.type.name;              // "Circle"
echo s.type.size;              // sizeof(Circle variant)

if s.type == Shape.Circle => handle_circle(s);
```

### Compile-Time Type Validation

```rs
@comptime
let assert_stack_safe: (Type) -> void = t -> {
    if t.size > 64 => panic "Type exceeds safe stack frame size!";
};

@assert assert_stack_safe(PacketHeader);   // validated at compile time

// Generic bound using Type:
let alloc: (t: Type) -> ptr = @inline {
    allocate_bytes(it.size)
};

let p := alloc(Point);     // allocates sizeof(Point) bytes
let n := alloc(u64);       // allocates sizeof(u64) bytes
```

---

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

## 12. Generics

```rs
// Simple generic:
let wrap: <T> (T) -> Option<T> = { .Some(it) };

// Single bound:
let show_all: <T: Display> (Arr<T>) -> string = items -> {
    items |> map(i -> i.show) |> join(", ")
};

// Multiple bounds with +:
let debug_log: <T: Display + Hash> (T) -> void = val -> {
    echo format("[{}] hash={}", val.show, val.hash)
};

// Generic type definition:
def Pair<A, B> = prod { first: A, second: B };

def Either<L, R> = sum {
    Left(L),
    Right(R),
};

// Generic with trait bounds on the type:
def @impl(Functor) Result<T, E> = extend {
    self: sum { Ok(T), Err(E) };

    map = <A, B> (f, r) -> {
        match r [
            .Ok(v)  => .Ok(f(v))
            .Err(e) => .Err(e)
        ]
    };
};

// Higher-kinded generics (container parameterizing an inner type):
let lift: <F: Functor, A, B> (def (A) -> B, F<A>) -> F<B> = (f, fa) -> {
    F.map(f, fa)
};
```

---

## 13. Pipe Operators & Data Flow

Pipes are the primary data transformation and composition tool in Element. All pipe
operators sit at **precedence level 0**, below all arithmetic and comparison operators.

### Positional Pipe `|>`, `|>>`, `|>>>`, `|>>>>`

Passes the left-hand side as the Nth argument to the right-hand function. The number of
`>` symbols (minus one) indicates which argument position receives the piped value.
Capped at 4 arrows:

```rs
a |>   f(b, c)    // f(a, b, c)    — a as first arg
a |>>  f(b, c)    // f(b, a, c)    — a as second arg
a |>>> f(b, c)    // f(b, c, a)    — a as third arg

// Common examples:
users |> filter(active)                   // filter(users, active)
value |>> divide(100.0)                   // divide(100.0, value)
item  |>>> insert(db, "users_table")      // insert(db, "users_table", item)
```

### Placeholder Pipe `|~`

`_` marks exactly where the piped value(s) land. The count of `_` must precisely match
the number of values being piped — a mismatch is a **compile error**:

```rs
x, y    |~ f(_, _, z, 4)      // f(x, y, z, 4)
x, y, z |~ f(0, _, _, _)      // f(0, x, y, z)
value   |~ transform(pre, _, post)  // transform(pre, value, post)

// Compile error — 2 values piped, 3 underscores:
x, y |~ f(_, _, _, 4)   // error: 2 piped values but 3 placeholders
```

### Value Group Spreading

```rs
let coords := (10, 20);

coords     |> draw_at(style);    // draw_at(coords, style) — group as one value
coords...  |> draw_at(style);    // draw_at(10, 20, style) — unpacked
```

### Monadic Pipeline `|>?`

Threads a monadic value through a chain at **precedence level 1**. If an error/failure
variant is encountered, it stays wrapped and propagates through the rest of the chain —
the outer function continues executing with the wrapped result:

```rs
let report := user_id
    |>  fetch_user        // (int) -> Result<User, string>
    |>? load_account      // (User) -> Result<Account, string>
    |>? get_balance;      // (Account) -> Result<Balance, string>

// report: Result<Balance, string>
// outer function continues here regardless of success or failure:
log_attempt(report);
report
```

### `$` — Low-Precedence Application

`$` sits at **precedence level -1** (below all pipes). The pipeline on the right resolves
entirely before `$` applies the outermost function. Right-associative:

```rs
echo $ raw_data |> parse_csv |> summarize;
// = echo(raw_data |> parse_csv |> summarize)

// Chained $:
f $ g $ x    // = f(g(x))
```

### Pipeline Examples

```rs
// Standard transformation pipeline:
let report: string =
    raw_data
    |> parse_csv
    |> filter(valid_row)
    |> map(to_user)
    |> sort_by(u -> u.name)
    |> format_table;

// Monadic pipeline for fallible operations:
let result: Result<Profile, string> =
    request.body
    |>  parse_json
    |>? validate_schema
    |>? authenticate(token)
    |>? fetch_profile;

// Using $ to lead with the sink:
echo $ users |> filter(active) |> map(u -> u.name) |> join(", ");
```

---

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

## 15. Operator System

All operators in Element are functions. `a + b` desugars to `add(a, b)`. There is no
special-cased operator syntax at the language level. Operators are distinguished purely
by their fixity declaration.

### Fixity — Three Independent Concerns

**`@position`** — where the symbol sits relative to its arguments:
- `infix`   — symbol between two args: `a + b`
- `prefix`  — symbol before arg(s): `-a`, `!b`
- `postfix` — symbol after arg(s): `a?`, or RPL-style `a b +`

**`@bind`** — maps positional sources to function parameters:

```rs
@bind(left->a, right->b)        // infix standard: left=a, right=b
@bind(right->a, left->b)        // infix flipped
@bind(left(1)->a, left(2)->b)   // binary postfix / RPL stack style
@bind(right->a)                 // prefix unary
@bind(left->a)                  // postfix unary
```

**`@assoc`** — how chains of the same operator group:
- `left`  — `a + b + c` = `(a + b) + c`
- `right` — `a $ b $ c` = `a $ (b $ c)`

### Defining an Operator

```rs
// Standard infix addition for a custom type:
@operator
let @position(infix) @bind(left->a, right->b) @assoc(left) @precedence(6)
(+): (MyNum, MyNum) -> MyNum = { MyNum(a.val + b.val) };

// RPL-style postfix binary (both args from the left):
@operator
let @position(postfix) @bind(left(1)->a, left(2)->b) @assoc(left) @precedence(6)
(+): (MyNum, MyNum) -> MyNum = { MyNum(a.val + b.val) };
// Usage: mynum1 mynum2 +

// Prefix negation:
@operator
let @position(prefix) @bind(right->a) @precedence(9)
(-): (MyNum) -> MyNum = { MyNum(-a.val) };

// Custom postfix operator:
@operator
let @position(postfix) @bind(left->a) @precedence(3)
(!): (Arr<T>) -> bool = { a.length == 0 };
// Usage: arr!  — true if empty
```

### Backtick Infix

Any two-argument function can be used as an infix operator by wrapping it in backticks.
Inherits the function's declared `@precedence` if present, otherwise defaults to level 4:

```rs
let result := a \compare_by\ key;
let groups := xs \partition\ is_even;
let merged := list_a \zip_with\ list_b;
```

### Master Precedence Table

| Level | Operators | Assoc | Description |
|---|---|---|---|
| **10** | `id`, `echo`, `panic`, `cast`, `todo` | Right | Parenthesis-free prefix intrinsics |
| **9** | `-`, `!`, `~` | Right | Unary negation, logical NOT, bitwise NOT |
| **8** | `#` | Right | N-times function application |
| **7** | `*`, `/`, `%` | Left | Multiplicative arithmetic |
| **6** | `+`, `-`, `<>` | Left | Additive arithmetic and monoid append |
| **5** | `<<<.`, `>>>.` | Left | Bitwise left and right shifts |
| **4** | `>>`, `<<` | Left | Left-to-right and right-to-left function composition |
| **4** | `` \f\ `` | Dynamic | Backtick-wrapped binary function |
| **4** | `<$>`, `<*>` | Left | Functor map and applicative apply |
| **3** | `<`, `>`, `<=`, `>=`, `==`, `!=` | Left | Relational comparisons |
| **2** | `&&`, `\|\|`, `<\|>` | Left | Logical operators and alternative |
| **1** | `\|>?` | Left | Monadic dependent pipeline |
| **0** | `\|>`, `\|>>`, `\|>>>`, `\|>>>>` | Left | Positional pipes |
| **0** | `\|~` | Left | Placeholder pipe |
| **-1** | `$`, `$$`, `$$$` | Right | Positional right-to-left application |
| **-1** | `$~` | Right | Placeholder right-to-left application |
| **-2** | `?` | N/A | Postfix early escape / error handler |

---

## 16. Standard Combinators

All combinators are regular operator-functions importable from `std.combinators`. Nothing
in this module is special-cased by the compiler — every combinator is implemented in
pure Element using the fixity and operator systems.

```rs
import std.combinators;                         // import all
import std.combinators { (>>), flip, on, id };  // selective import
```

### Application Operators

```rs
// $ — low-precedence forward application:
echo $ users |> filter(active) |> map(u -> u.name);
// = echo(users |> filter(active) |> map(u -> u.name))

// $$ — second-argument application:
divide $$ 2.0 $ calculate();
// = divide(calculate(), 2.0)

// $$$ — third-argument application:
insert $$$ db_handle $ build_record(data);

// $~ — placeholder right-to-left:
format $~ "Hello, _!" $ get_name();

// & — flipped application (value-first, single step):
42 & double & inc & show;
// = show(inc(double(42)))
```

### Composition Operators

```rs
// >> — left-to-right function composition:
let process_user :=
    validate_email
    >> normalize_name
    >> check_permissions
    >> create_account;

new_user |> process_user;

// << — right-to-left composition (mathematical notation):
let render := format_html << sort_by_date << filter_published;
// render(x) = format_html(sort_by_date(filter_published(x)))
```

### Core Combinator Functions

```rs
// id — identity (I combinator):
let transform := if debug_mode { log_pass } else { id };
data |> transform |> next_step;

// const — always return first arg, discard second (K combinator):
arr |> map(const(0, _));               // [0, 0, 0, ...]
button.on_click(const((), _));         // no-op handler

// flip — swap first two argument positions (C combinator):
let divide_by_two := flip(divide)(2.0, _);
values |> map(divide_by_two);

// on — lift a binary function through a key extraction:
let sort_by_age   := sort \on\ (p -> p.age);
let compare_names := compare \on\ (p -> p.name);
people |> sort_by(compare_names);

// fix — Y combinator for programmatic anonymous recursion:
let factorial := fix((self, n) -> {
    if n == 0 => 1
    else      => n * self(n - 1)
});
```

### Higher-Kinded Operators

```rs
// <$> — functor map:
double  <$> .Some(21)    // .Some(42)
double  <$> .Ok(21)      // .Ok(42)
(+1)    <$> [1, 2, 3]   // [2, 3, 4]

// <*> — applicative apply:
.Some(double) <*> .Some(5)    // .Some(10)
.None         <*> .Some(5)    // .None

// >>= — monadic bind:
get_user(id) >>= load_profile >>= fetch_avatar >>= resize(128, _);

// <|> — alternative / fallback:
from_env("PORT") <|> from_config("port") <|> .Ok(8080);

// <> — monoid append (also available as operator at precedence 6):
"Hello" <> ", " <> "world"    // "Hello, world"
[1, 2]  <> [3, 4]             // [1, 2, 3, 4]

// # — N-times application (at precedence 8):
x |> 3#double    // double(double(double(x)))
x |> 0#f         // id(x) — zero applications = identity
```

### Point-Free Style

The combination of `>>`, `flip`, `on`, and partial application enables concise
point-free function definitions:

```rs
// Pointed style:
let summarize := (xs: Arr<User>) -> {
    xs |> filter(u -> u.active) |> map(u -> u.name) |> sort |> join(", ")
};

// Point-free style:
let summarize :=
    filter(active_user)
    >> map(u -> u.name)
    >> sort
    >> join(", ");
```

---

## 17. Error Handling

Element provides two complementary error handling mechanisms with a clear design boundary
between functional and imperative styles.

### `?` — Three Forms

`?` is an operator defined with `@position(postfix)`. All three forms desugar to function
calls, making them user-overridable for custom types.

**Postfix — propagate error upward, exit current function:**

```rs
let user:    User    = fetch_user(id)?;
let profile: Profile = load_profile(user)?;
let avatar:  Arr<u8> = fetch_avatar(profile.avatar_url)?;
// If any step returns Err, the function returns that Err immediately
```

**Infix with block — handle error in place, no binding:**

```rs
let config := load_config("app.json")? {
    echo "Config file missing — using defaults";
    default_config()
};
```

**Infix with typed error capture:**

```rs
let data := fetch_remote(url)?(e: NetworkError) {
    log_error(format("Network failure: {}", e.show));
    .Ok(cached_fallback())
};
```

### `|>?` vs `?` — Design Boundary

| | `\|>?` | `?` |
|---|---|---|
| Style | Functional, pipeline | Imperative, sequential |
| On error | Stays wrapped, chain continues | Unwinds frame, function returns early |
| Result type in binding | `Result<T, E>` (wrapped) | `T` (unwrapped value) |
| Use when | Composing fallible operations | Sequential steps where failure should abort |

```rs
// Functional — result stays wrapped throughout:
let report: Result<Report, string> =
    user_id
    |>  fetch_user
    |>? load_account
    |>? compute_report;

log_attempt(report);    // always runs
report                  // returns Result

// Imperative — each step unwraps or the function returns early:
let user:    User    = fetch_user(user_id)?;
let account: Account = load_account(user)?;
let report:  Report  = compute_report(account)?;

notify_success(user.id);   // only runs if all steps succeeded
.Ok(report)
```

---

## 18. The use Keyword — Callback Flattening

`use` replaces `let` in a binding to signal that the right-hand function takes a
**callback as its final argument**. `use` captures the entire remainder of the current
block as an implicit closure passed to that final argument, eliminating callback nesting.

```rs
// Without use — deeply nested:
database.connect(config, conn -> {
    conn.begin_transaction(session -> {
        open("import.csv", file -> {
            file.read() |> parse_csv |> insert_rows(session)
        })
    })
});

// With use — flat:
use conn    := database.connect(config);
use session := conn.begin_transaction();
use file    := open("import.csv");

file.read() |> parse_csv |> insert_rows(session);
```

The desugaring of `use conn := f(args)` is:
```rs
f(args, conn -> { /* rest of block */ })
```

**Without a binding** (when the yielded value is not needed):

```rs
use acquire_lock(mutex);
critical_section();
// desugars to: acquire_lock(mutex, () -> { critical_section() })
```

**Resource management** — the called function handles cleanup after the callback:

```rs
use file := open("data.txt");
let content := file.read();
content |> process |> echo;
// open() closes file after callback returns
```

**Actor spawning:**

```rs
use pid := spawn();
receive [
    ("ping", sender)  => sender |> send("pong")
    ("stop")          => return
    (..)              => echo "unknown message"
];
```

**Multiple `use` bindings** stack naturally, with each capturing the rest of the block:

```rs
use conn    := database.connect(config);
use session := conn.begin_transaction();
use lock    := acquire_write_lock(resource);

// All three resources active here, all cleaned up in reverse order
do_work(session, lock);
```

---

## 19. Dot Access Model

Dot notation in Element has exactly two meanings. It is **not** method-call syntax:

- `foo.field`    — direct field access on a `prod` type
- `foo.computed` — access a zero-argument computed property from `extend`,
                   with `self` implicitly bound to `foo`

**There are no method calls.** Functions requiring arguments are invoked via pipes or
direct application:

```rs
// NOT valid:
user.format("json")         // no method calls with arguments

// Valid alternatives:
format(user, "json")
user |> format("json")
format_json(user)
```

### Computed Properties via `extend`

Computed properties follow the thunk model — evaluated on first access, cached:

```rs
def @impl(Display, Hash) User = extend {
    self: prod { id: int, name: string, email: string };

    // Computed properties — zero-argument, cached after first access:
    show:     string = { format("User({})", self.name) };
    domain:   string = { self.email |> split("@") |> last };
    initials: string = { self.name |> words |> map(w -> w |> first_char) |> join("") };
    hash:     u64    = { hash_combine(self.id.hash, self.name.hash) };
};

let u := User(1, "Alice Smith", "alice@example.com");
echo u.show;      // "User(Alice Smith)"
echo u.domain;    // "example.com"  — cached
echo u.initials;  // "AS"           — cached
echo u.hash;      // computed once
```

### The `Time` Type — Canonical Example

```rs
def @impl(Display, Eq, Ord<Time>) Time = extend {
    self: int;   // unix timestamp in seconds — the only stored value

    ms:      int = { self * 1_000 };
    us:      int = { self * 1_000_000 };
    ns:      int = { self * 1_000_000_000 };
    s:       int = { self };
    minutes: int = { self / 60 };
    hours:   int = { self / 3_600 };
    days:    int = { self / 86_400 };

    show:    string = { format("{}s ({}ms)", self.s, self.ms) };
    equals   = (a, b) -> a.self == b.self;
    compare  = (a, b) -> a.self - b.self;
};

let t := now();
echo t.ms;       // no method call — reads like a field, computes once
echo t.days;     // same
echo t.show;     // "1718000000s (1718000000000ms)"
```

---

## 20. Metaprogramming Pipeline

Element splits compilation into isolated, predictable processing phases. Each phase has
strict boundaries preventing information leakage backward in the pipeline.

### Phase 1: Token Macros (`@macro`)

Token macros run on raw, unparsed token streams before the AST is constructed. They are
declared with the `@macro` declaration-level attribute and invoked with the `!` suffix:

```rs
@macro
let json!: (TokenStream) -> TokenStream = stream -> {
    let validated := validate_json_tokens(stream);
    transform_to_element_ast(validated)
};

@macro
let sql!: (TokenStream) -> TokenStream = stream -> {
    parse_sql(stream) |> generate_prepared_statement
};

// Usage at call sites:
let config := json!{ "host": "localhost", "port": 8080 };
let query  := sql!{ SELECT * FROM users WHERE active = true };
```

**Fast-path optimization**: if a source file contains no `!` invocations, Phase 1 is
skipped entirely and source text streams directly to the AST generator.

**Isolation boundary**: token macros cannot be called inside `@comptime` blocks.
Cross-phase calls are a compile error.

### Phase 3: Compile-Time Element Execution (`@comptime`)

`@comptime` elements execute entirely inside the compiler's internal SLUR VM during type
checking. Their return values are substituted directly into the binary:

```rs
@comptime
let generate_op_table: (Arr<string>) -> Arr<(string, int)> = ops -> {
    ops |> map_indexed((op, i) -> (op, i))
};

@comptime
let validate_packet_layout: (Type) -> void = t -> {
    if t.size > 1500 => panic format("{} exceeds MTU", t.name);
    if t.align != 8  => panic format("{} must be 8-byte aligned", t.name);
};

// Invoked at compile time:
@assert validate_packet_layout(UdpPacket);

let @const OPS: Arr<(string, int)> = @run generate_op_table(["add", "sub", "mul"]);
```

### `@auto_impl` — Derived Trait Implementations

A built-in `@macro`-based mechanism for automatically generating `extend` blocks for
type traits with mechanical field mappings:

```rs
// Derive Monoid with explicit field values:
@auto_impl(Monoid(empty: 0, combine: (+)))
def Counter = prod { count: int };

// Derive Eq and Ord using existing functions:
@auto_impl(Eq(equals: (==)), Ord(compare: int_compare))
def Score = prod { value: int };

// Derive Display with a format string:
@auto_impl(Display(show: "Score({})" % self.value))
def Score = prod { value: int };
```

### Statement-Level Directives (Summary)

```rs
@embed("path/to/file")     // embed file contents as Arr<u8>
@run fn_call()              // execute function at compile time
@insert "other.elem"        // textual inclusion
@if CONDITION { ... }       // conditional compilation
@assert CONDITION           // compile-time assertion

// Right-side forms:
let DATA: Arr<u8> = @embed("assets/icon.png");
let TABLE: Arr<int> = @run build_lookup_table();
```

---

## 21. Concurrency Model *(Outline — Full Spec Pending)*

Element's concurrency model is based on isolated actors communicating via typed mailboxes.
Actors share no memory. All communication is message-passing via `send` and `receive`.

```rs
// Spawning an actor with use:
use pid := spawn();
receive [
    ("ping", sender) => sender |> send("pong")
    ("echo", msg)    => echo msg
    ("stop")         => return
    (..)             => echo "unknown message"
];

// Sending messages:
pid |> send("ping", self);
pid |> send("stop");
```

Coroutines with `yield` usable as lazy streams are planned. Full mailbox typing, actor
supervision trees, and the interaction between the actor model and `use` are pending
specification.

---

## 22. Build System & Tooling

Build scripts (`build.elem`) are evaluated at compile time. `@lazy` elements serve as
thunks representing build targets, evaluated only when invoked:

```rs
import std.compiler as _;

let @pub @lazy build_frontend = {
    let opts := default_opts
        |> set_output(.wasm, "dist/frontend.wasm")
        |> add_lib("web")
        |> set_opt_level(3);

    async compile(opts);
    wait
};

let @pub @lazy build_backend = {
    let opts := default_opts
        |> set_output(.native, "dist/server")
        |> add_lib("http")
        |> add_lib("database");

    async compile(opts);
    wait
};

let @pub @lazy build_all = {
    build_frontend;
    build_backend
};
```

**`sh-mode`**: when the compiler flag `sh-mode` is active, unresolved identifiers are
treated as shell commands, blending OS-level scripting with Element's functional pipelines.
Sandboxing model is pending specification.

---

## 23. Low-Level Control

### Inline Assembly

Standard library and performance-critical code can drop directly into SLUR stack-machine
instructions using `asm {}` blocks:

```rs
let fast_add: (int, int) -> int = (a, b) -> {
    asm { add }
};

let swap_top: (int, int) -> (int, int) = (a, b) -> {
    asm { swap }
};

let mem_copy: (ptr, ptr, int) -> void = (dst, src, len) -> {
    asm { memcpy }
};
```

Inline assembly is reserved for standard library authors and low-level systems code.
Normal application code should use the standard library abstractions instead.

### Unparenthesized Intrinsics

Core VM intrinsics act as unary keywords at **precedence level 10**, permitting
parenthesis-free call syntax:

```rs
echo value;
echo "hello, world";
panic "critical failure: state corrupted";
todo;          // marks unimplemented code; panics at runtime if reached
cast<int>(float_value);
```

These are the **only** identifiers in Element callable without parentheses. All
user-defined functions require explicit call syntax. The precedence 10 placement means
intrinsic calls resolve before all binary operators.

---

## 24. Open / Not Yet Specified

The following areas have been identified but not yet formally specified. They will be
addressed in future chapters of this document.

| Area                                 | Status          | Notes                                                 |
| ------------------------------------ | --------------- | ----------------------------------------------------- |
| Memory model                         | 🔲 Not started  | Ownership, allocation strategy, GC vs arena           |
| Module system                        | 🔲 Not started  | Declaration, imports, namespacing, visibility scoping |
| Full numeric primitive tower         | 🔲 Not started  | Overflow behaviour, casting rules, literals           |
| Async / await                        | 🔲 Not started  | General model vs build-system-only                    |
| Coroutines / yield-as-stream         | 🔲 Concept only | Interaction with actor model                          |
| Actor mailbox typing                 | 🔲 Not started  | Typed channels, supervision trees                     |
| `sh-mode` sandboxing                 | 🔲 Concept only | Security model for shell integration                  |
| Error type hierarchy                 | 🔲 Deferred     | Rust-like but more ergonomic; no `Box<dyn Error>`     |
| String interpolation                 | 🔲 Not started  | `format("Hello {}", name)` or `"Hello {name}"`        |
| Full standard library surface        | 🔲 Not started  | Collections, I/O, networking, math                    |
| Import / module syntax               | 🔲 Not started  | `import`, namespacing, re-exports                     |
| Metaprogramming Phase 2              | 🔲 Not started  | The gap between token macros and comptime             |
| Lifetime / borrow semantics          | 🔲 Not started  | If applicable given target VM model                   |
| Variadic functions                   | 🔲 Not started  | `Arr<T>` splat vs true variadic                       |
| Operator resolution for custom types | 🔲 Not started  | How `+` on unknown types is resolved at compile time  |
