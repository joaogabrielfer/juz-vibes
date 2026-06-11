# std::prelude

The prelude is the standard user-level surface available without explicit imports.

Element has no intrinsic value-level operators. Arithmetic, comparison, logic, append,
output, panic, casting, yield, collection handlers, and recovery names are all modeled as
ordinary declarations in the default environment.

The parser still knows the notation shapes and precedence levels. The prelude supplies
the default declarations those shapes resolve to.

## Draft Module Surface

```rust
module std::prelude

import std::core::types {
    Option,
    Either,
    Fallible,
    Error,
    SourceLoc,
    ErrorFrame,
}

import std::core::traits {
    Display,
    Eq,
    Ord,
    Hash,
    Clone,
    IntoFallible,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,
    BitNot,
    BitShiftLeft,
    BitShiftRight,
}

import std::effects {
    Yield,
    IO,
    yield,
    stream,
    collect,
    recover,
    resume,
}
```

This import list is conceptual. The exact module path and whether the implementation
uses re-export syntax are not finalized.

The `std` package root is addressable by default through explicit paths such as
`std::fs::open`. The prelude controls only unqualified names such as `echo`, `panic`, and
default notation declarations. Disabling the prelude does not remove explicit access to
`std::...` paths.

## Low-Level Output and Exit

`print` is the lowest-level output surface currently documented. It writes one string to
the default program output stream. OS interop, terminal configuration, stderr/stdout
splitting, buffering, and host capabilities are intentionally deferred.

```rust
let print: string -> unit
let exit: int -> never
```

`exit(1)` terminates the current program with status code `1`.

## Default Named Notations

```rust
@notation
@position(prefix) @bind(right->text) @precedence(10) @adjacent(right)
let (print): string -> unit

@notation
@position(prefix) @bind(right->value) @precedence(10) @adjacent(right)
let (echo): <T: Display> (value: T) -> unit = {
    print Display::show(value)
}

@notation
@position(prefix) @bind(right->value) @precedence(10) @adjacent(right)
let (panic): <T: Display> (value: T) -> never = {
    print Display::show(value);
    exit(1)
}

@notation
@position(prefix) @precedence(10)
let (todo): unit -> never = {
    panic "not implemented"
}

@notation
@position(prefix) @bind(right->value) @precedence(10)
let (cast): <T> (value: _) -> T = {
    unsafe_cast<T>(value)
}
```

`print`, `echo`, `panic`, `todo`, and `cast` are keyword-like because they are prefix
notations in the prelude, not because they are special parser forms.

`panic` has no separate VM interop hook in this revision. It prints the panic message and
terminates the current program with exit status `1`.

## Default Arithmetic Notations

The following signatures describe the generic shape. Concrete numeric implementations
may be primitive-backed, VM-backed, or trait-resolved.

```rust
@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(6) @adjacent(left, right)
let (+): <T: Add<T>> T, T -> T

@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(6) @adjacent(left, right)
let (-): <T: Sub<T>> T, T -> T

@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(7) @adjacent(left, right)
let (*): <T: Mul<T>> T, T -> T

@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(7) @adjacent(left, right)
let (/): <T: Div<T>> T, T -> T

@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(7) @adjacent(left, right)
let (%): <T: Rem<T>> T, T -> T
```

Open question:
Overflow behavior, checked conversion APIs, and detailed numeric diagnostics remain
open. The arithmetic trait family is documented in [[std-core-traits]].

## Default Unary Notations

```rust
@notation
@position(prefix) @bind(right->value) @precedence(9) @adjacent(right)
let (-): <T: Neg<T>> T -> T

@notation
@position(prefix) @bind(right->value) @precedence(9) @adjacent(right)
let (!): bool -> bool

@notation
@position(prefix) @bind(right->value) @precedence(9) @adjacent(right)
let (~): <T: BitNot<T>> T -> T
```

## Default Comparison Notations

```rust
@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(3) @adjacent(left, right)
let (==): <T: Eq<T>> T, T -> bool

@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(3) @adjacent(left, right)
let (!=): <T: Eq<T>> T, T -> bool

@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(3) @adjacent(left, right)
let (<): <T: Ord<T>> T, T -> bool

@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(3) @adjacent(left, right)
let (<=): <T: Ord<T>> T, T -> bool

@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(3) @adjacent(left, right)
let (>): <T: Ord<T>> T, T -> bool

@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(3) @adjacent(left, right)
let (>=): <T: Ord<T>> T, T -> bool
```

## Default Logical Notations

```rust
@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(2) @adjacent(left, right)
let (&&): bool, bool -> bool

@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(2) @adjacent(left, right)
let (||): bool, bool -> bool
```

Open question:
Short-circuiting needs a precise lowering rule. If `&&` and `||` are ordinary eager
functions, they cannot short-circuit. If they short-circuit, they either need thunked
right operands, compiler-recognized evaluation behavior for specific prelude notations, or
an explicit lazy-argument notation model.

## Default Append and Shift Notations

```rust
@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(6) @adjacent(left, right)
let (<>): <T: Append<T>> T, T -> T

@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(5) @adjacent(left, right)
let (<<<.): <T: BitShiftLeft<T>> T, int -> T

@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(5) @adjacent(left, right)
let (>>>.): <T: BitShiftRight<T>> T, int -> T
```

## Handler Names in the Prelude

`yield`, `stream`, `collect`, `recover`, and `resume` are listed in the prelude because
the current language examples use them without explicit imports.

```rust
let squares := collect {
    for n in range(0, 10) {
        yield n * n
    }
}

let config := (recover load_config(path) [
    .FileMissing(e) => resume .UsePath(default_config_path)
])?
```

The declarations themselves are specified in [[std-effects]].

## Import-Only Notations

Not every standard notation belongs in the prelude.

The following remain import-only through [[std-functional]]:

- `>>`
- `<<`
- `>>=`
- `<$>`
- `<*>`
- `<|>`
- `&`
- `#`
