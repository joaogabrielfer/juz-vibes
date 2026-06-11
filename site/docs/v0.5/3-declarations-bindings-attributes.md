## 3. Declarations, Bindings & Attributes

### `let` Forms

`let` declares value-level elements. `::` is reserved for standalone type signatures and
cannot be combined with `=`.

```rs
// Invalid - cannot combine :: and =:
let map :: (int) -> int = x -> x * 2

// Valid - separate type signature and value definition:
let map :: (int) -> int
let map = x -> x * 2

// Valid - inline annotated assignment:
let map: (int) -> int = x -> x * 2
```

Current `let` forms:

| Form | Syntax | Meaning |
|---|---|---|
| Type signature only | `let name :: type` | Forward declaration, no value yet |
| Value only | `let name = value` | Value definition, type inferred |
| Inferred binding shorthand | `let name := value` | Value definition, type inferred |
| Annotated assignment | `let name: type = value` | Inline type plus value |

`let name :: type` declarations are valid only at module scope. Inside `{}` blocks, values
must be defined in dependency order.

`:=` remains accepted in v0.5 examples as the inferred binding shorthand. The future style
preference between `let name = value` and `let name := value` is tracked in
[[open-questions]].

### `mut`

`mut` immediately follows `let` and marks a binding as mutable. It precedes inline
attributes:

```rs
let mut score: int = 0
let mut @pub counter: int = 0
```

Attempting to reassign a non-`mut` binding is a type error.

### `def` Forms

`def` is exclusively for type declarations and always uses `::`. There is no
`def Name = ...` form.

```rs
def Point   :: prod { x: int, y: int }
def Shape   :: sum  { Circle { r: float }, Rect { w: float, h: float } }
def Status  :: enum<int> { Active = 1, Inactive, Pending }
def UserId  :: int
def Handler :: def((Request) -> Response)
def Display :: trait { show: (Self) -> string }
```

### Attribute Placement

Attributes have three placement rules:

- Declaration-level attributes go on the line above `def` declarations.
- Inline `let` attributes stay after `let`, after `mut` if present.
- Under `@notation`, notation-configuration attributes (`@position`, `@bind`,
  `@assoc`, `@precedence`, `@adjacent`) go on the line above the `let` declaration and
  are written on a single line.

```rs
@auto_impl(Eq, Ord, Hash, Clone)
def Point :: prod { x: int, y: int }

@impl(Display)
def Point :: extends {
    self: prod { x: int, y: int },
    show = "{self.x}, {self.y}",
}

let @impl(Display) show = self: Point -> "{self.x}, {self.y}"
```

### Declaration-Level Attributes

Declaration-level attributes modify the declaration that follows:

```rs
@macro
let json!: (TokenStream) -> TokenStream = stream -> { ... }

@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(6) @adjacent(left, right)
(+): (MyInt, MyInt) -> MyInt = { MyInt(a.val + b.val) }

@comptime
let validate_layout: (Type) -> void = t -> {
    if t.size > 128 => panic "exceeds stack frame limit";
}
```

For `def`, `@impl` and `@auto_impl` are declaration-level attributes:

```rs
@impl(Monad, Applicative, Functor)
def Option<T> :: extends { ... }

@auto_impl(Monoid(empty: 0, combine: (+)))
def Score :: prod { value: int }
```

### Inline `let` Attributes

Inline attributes annotate a `let` binding:

| Attribute | Meaning |
|---|---|
| `@pub` | Export from module |
| `@pub(module)` | Visible across all files in the current module |
| `@pub(project)` | Visible inside the current project, not external packages |
| `@const` | Compile-time constant |
| `@inline` | Inline at every call site |
| `@lazy` | Remove thunk cache; re-evaluate on every bare access |
| `@impl(Trait)` | Bind as an implementation member of the named trait |
| `@extern("symbol")` | Link to an external symbol by name |
| `@deprecated("msg")` | Emit a deprecation warning at usage sites |
| `@test` | Mark as a unit test; excluded from non-test builds |
| `@align(n)` | Memory alignment hint in bytes |
| `@packed` | No struct padding |
| `@memoize` | Cache pure function result |

Under `@notation`, additional declaration-level attributes configure notation behavior:

| Attribute | Meaning |
|---|---|
| `@position(infix/prefix/postfix)` | Where the symbol sits relative to its arguments |
| `@bind(left->a, right->b)` | Map positional sources to parameter names |
| `@assoc(left/right)` | Chain grouping direction |
| `@precedence(n)` | Numeric precedence level |
| `@adjacent(left/right)` | Permit no-whitespace adjacency on bound sides |

`@adjacent(...)` never allows token splitting. For example, `echofoo` is always one
identifier token and is never parsed as `echo foo`.

### Constants

The retired JAI-style `let MAX :: 100` syntax is invalid. `::` now means type annotation
exclusively.

```rs
// Invalid:
let MAX :: 100

// Valid:
let @const MAX: int = 100
let @pub @const VERSION: string = "1.0.0"
let @const BUILD_DIR := "./build/"
```

### Statement-Level Directives

`@` also serves as a statement-scope directive, primarily for compile-time operations:

```rs
@embed("shaders/frag.spv")
@run validate_config()
@insert "generated/routes.jz"
@if TARGET == .wasm { ... }
@assert sizeof(PacketHeader) == 32

let SHADER: Arr<u8>  = @embed("shaders/frag.spv")
let PRIMES: Arr<int> = @run generate_primes(1000)
```
