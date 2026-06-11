## 16. std.functional

`std.functional` provides import-only functional notations and helper elements. Nothing in
this module is compiler-special-cased.

```rs
import std.functional
import std.functional { (>>), (>>=), (<$>), flip, on, id, fix }
```

### Import-Only Notations

These notations are defined in `std.functional` and require import.

| Notation | Role |
|---|---|
| `>>` | Left-to-right function composition |
| `<<` | Right-to-left function composition |
| '>>=' | Monadic bind |
| `<$>` | Functor map |
| `<*>` | Applicative apply |
| `<\|>` | Alternative/fallback choice |
| `&` | Forward application chaining |
| `#` | N-times function application |

### Import-Only Helper Elements

| Element | Role |
|---|---|
| `id` | Identity function |
| `const` | Constant function builder |
| `flip` | Reversed argument order |
| `on` | Project then compare/combine |
| `fix` | Fixed-point combinator |

### Usage Examples

```rs
let process_user :=
    validate_email
    >> normalize_name
    >> check_permissions
    >> create_account

new_user |> process_user

let render := format_html << sort_by_date << filter_published

let passthrough := id
data |> passthrough |> next_step

arr |> map(const(0, _))
button.on_click(const((), _))

let divide_by_two := flip(divide)(2.0, _)
values |> map(divide_by_two)

let sort_by_age   := sort `on` (p -> p.age)
let compare_names := compare `on` (p -> p.name)
people |> sort_by(compare_names)

let factorial := fix((self, n) -> {
    if n == 0 => 1
    else      => n * self(n - 1)
})
```

### Kinded Functional Notations

`<$>` is infix functor mapping and desugars to `Functor.map(f, fa)`. It preserves context
shape and transforms only contained values. The formal kinded trait model is specified in
[[25-kinds-higher-kinded-types]].

```rs
double <$> .Some(21)
double <$> .Ok(21)
(+1)   <$> [1, 2, 3]

.Some(double) <*> .Some(5)
.None         <*> .Some(5)

get_user(id) >>= load_profile >>= fetch_avatar >>= resize(128, _)

from_env("PORT") <|> from_config("port") <|> .Ok(8080)

x |> 3#double
x |> 0#f
```

### '>>=' and `|>?`

'>>=' and `|>?` can express similar chaining behavior, but they serve different roles:

- `|>?` is part of the core pipe family and is restricted to `Fallible`-implementing
  types.
- '>>=' is a general monadic bind notation imported from `std.functional`.

Prefer `|>?` for error propagation in pipelines and '>>=' when working directly in monad
abstractions.
