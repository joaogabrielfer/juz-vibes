## 16. Standard Combinators

All combinators are regular operator-functions importable from `std.combinators`. Nothing
in this module is special-cased by the compiler.

```rs
import std.combinators
import std.combinators { (>>), flip, on, id }
```

### Application Operators

```rs
echo $ users |> filter(active) |> map(u -> u.name)

divide $$ 2.0 $ calculate()

insert $$$ db_handle $ build_record(data)

format $~ "Hello, _!" $ get_name()

42 & double & inc & show
```

### Composition Operators

```rs
let process_user :=
    validate_email
    >> normalize_name
    >> check_permissions
    >> create_account

new_user |> process_user

let render := format_html << sort_by_date << filter_published
```

### Core Combinator Functions

```rs
let transform := if debug_mode { log_pass } else { id }
data |> transform |> next_step

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

### Higher-Kinded Operators

`<$>` is infix functor mapping and desugars to `Functor.map(f, fa)`. It preserves context
shape and transforms only contained values. The formal kinded trait model is specified in
[[25-kinds-higher-kinded-types]].

```rs
double  <$> .Some(21)
double  <$> .Ok(21)
(+1)    <$> [1, 2, 3]

.Some(double) <*> .Some(5)
.None         <*> .Some(5)

get_user(id) >>= load_profile >>= fetch_avatar >>= resize(128, _)

from_env("PORT") <|> from_config("port") <|> .Ok(8080)

"Hello" <> ", " <> "world"
[1, 2]  <> [3, 4]

x |> 3#double
x |> 0#f
```

### Point-Free Style

```rs
let summarize := (xs: Arr<User>) -> {
    xs |> filter(u -> u.active) |> map(u -> u.name) |> sort |> join(", ")
}

let summarize :=
    filter(active_user)
    >> map(u -> u.name)
    >> sort
    >> join(", ")
```
