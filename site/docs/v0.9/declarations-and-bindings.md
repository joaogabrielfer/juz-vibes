# Declarations and Bindings

This chapter owns value declarations, type declarations, mutability, constants,
declaration attributes, and statement-level directives.

## `let` Forms

`let` declares value-level elements. `::` is reserved for standalone type signatures and
cannot be combined with `=`.

Invalid:

```rust
let map :: (int) -> int = x -> x * 2
```

Valid:

```rust
let map :: (int) -> int
let map = x -> x * 2

let map: (int) -> int = x -> x * 2
let map := x -> x * 2
```

Current forms:

| Form | Syntax | Meaning |
|---|---|---|
| Type signature only | `let name :: type` | Forward declaration, no value yet |
| Value only | `let name = value` | Value definition, type inferred |
| Inferred binding shorthand | `let name := value` | Value definition, type inferred |
| Annotated assignment | `let name: type = value` | Inline type plus value |

`let name :: type` declarations are valid only at module scope. Inside `{}` blocks,
values must be defined in dependency order.

The future style preference between `let name = value` and `let name := value` remains
open in [[open-questions]].

## `mut`

`mut` immediately follows `let` and marks a binding as mutable. It precedes inline
attributes.

```rust
let mut score: int = 0
let mut @pub counter: int = 0
```

Attempting to reassign a non-`mut` binding is a type error.

Inside `def ... :: prod` and the `self` layout of a 2-in-1 `extends` declaration, `mut`
marks a mutable stored subelement.

```rust
def Point :: prod {
    x: int,
    mut y: int,
    mut show: string = "{self.x}, {self.y}",
}
```

## `def` Forms

`def` is exclusively for type-level declarations and always uses `::`.

```rust
def Point   :: prod { x: int, y: int }
def Shape   :: sum  { Circle { r: float }, Rect { w: float, h: float } }
def Status  :: enum<int> { Active = 1, Inactive, Pending }
def UserId  :: int
def Handler :: def((Request) -> Response)
def Display :: trait { show: (Self) -> string }
```

There is no `def Name = ...` form.

## Attribute Placement

Attributes have three placement rules:

- Declaration-level attributes go on the line above `def` declarations.
- Inline `let` attributes stay after `let`, after `mut` if present.
- Under `@notation`, `@macro`, and bang-callable `@comptime`, call-shape attributes go on
  the line above the declaration and are written on one line.

```rust
@auto_impl(Eq, Ord, Hash, Clone)
def Point :: prod { x: int, y: int }

let @impl(Display) show = self: Point -> self.show
```

Declared attribute schemas, typed attribute payloads, and untyped metadata are specified
in [[attributes-and-metadata]].

## Declaration-Level Attributes

Declaration-level attributes modify the declaration that follows.

```rust
@macro
let json!: (TokenStream) -> TokenStream = stream -> { ... }

@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(6) @adjacent(left, right)
let (+): (MyInt, MyInt) -> MyInt = { MyInt(a.val + b.val) }

@attribute
def inspect :: attr {
    targets: [.subelement],
    args: prod { min: Option<float>, max: Option<float> }
}
```

For `def`, `@impl` and `@auto_impl` are declaration-level attributes.

```rust
def Option<T> :: extends {
    self: sum { Some(T), None },

    @impl(Functor) { ... }
    @impl(Applicative) { ... }
    @impl(Monad) { ... }
}

@auto_impl(Monoid(empty: 0, combine: (+)))
def Score :: prod { value: int }
```

## Inline `let` Attributes

Current inline attributes:

| Attribute | Meaning |
|---|---|
| `@pub` | Export from module |
| `@pub(module)` | Visible across all files in the current module |
| `@pub(project)` | Visible inside the current project, not external packages |
| `@const` | Compile-time constant |
| `@inline` | Inline at every call site |
| `@impl(Trait)` | Bind as an implementation member of the named trait |
| `@extern("symbol")` | Link to an external symbol by name |
| `@deprecated("msg")` | Emit a deprecation warning at usage sites |
| `@test` | Mark as a unit test; excluded from non-test builds |
| `@align(n)` | Memory alignment hint in bytes |
| `@packed` | No struct padding |
| `@memoize` | Cache pure function result |

Call-shape attributes used by notation, macros, and bang-callable comptime elements:

| Attribute | Meaning |
|---|---|
| `@position(infix/prefix/postfix)` | Where the symbol sits relative to its arguments |
| `@bind(left->a, right->b)` | Map operand sources to parameter names |
| `@assoc(left/right)` | Chain grouping direction |
| `@precedence(n)` | Numeric precedence level |
| `@adjacent(left/right)` | Permit no-whitespace adjacency on bound sides |

## Constants

The retired JAI-style `let MAX :: 100` syntax is invalid. `::` means type annotation
exclusively.

```rust
let @const MAX: int = 100
let @pub @const VERSION: string = "1.0.0"
let @const BUILD_DIR := "./build/"
```

## Statement-Level Directives

`@` also serves as a statement-scope directive for compile-time operations.

```rust
@run validate_config()
@insert "generated/routes.jz"
@if TARGET == .wasm { ... }
@assert sizeof(PacketHeader) == 32

let PRIMES: Arr<int> = @run generate_primes(1000)
let SHADER: Arr<u8> = embed! "shaders/frag.spv"
```
