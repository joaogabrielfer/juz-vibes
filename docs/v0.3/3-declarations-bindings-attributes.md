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

