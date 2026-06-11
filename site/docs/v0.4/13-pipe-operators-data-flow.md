## 13. Pipe Operators & Data Flow

Pipes are the primary data transformation and composition tool in Element. Positional
pipe operators sit at precedence level 0, below arithmetic and comparison operators.

### Positional Pipe `|>`, `|>>`, `|>>>`, `|>>>>`

The left-hand side is passed as the Nth argument to the right-hand function. The number of
`>` symbols minus one indicates which argument position receives the piped value. The
family is capped at four arrows.

```rs
a |>   f(b, c)    // f(a, b, c)
a |>>  f(b, c)    // f(b, a, c)
a |>>> f(b, c)    // f(b, c, a)

users |> filter(active)
value |>> divide(100.0)
item  |>>> insert(db, "users_table")
```

### Placeholder Pipe `|~`

`_` marks exactly where the piped value or values land. The count of `_` must precisely
match the number of values being piped.

```rs
x, y    |~ f(_, _, z, 4)
x, y, z |~ f(0, _, _, _)
value   |~ transform(pre, _, post)

// Compile error: 2 piped values but 3 placeholders.
x, y |~ f(_, _, _, 4)
```

### Value Group Spreading

```rs
let coords := (10, 20)

coords   |> draw_at(style)    // draw_at(coords, style)
coords.. |> draw_at(style)    // draw_at(10, 20, style)
```

### Monadic Pipeline `|>?`

`|>?` threads a monadic value through a chain at precedence level 1. If an error or
failure variant is encountered, it stays wrapped and propagates through the rest of the
chain. The outer function continues executing with the wrapped result.

```rs
let report := user_id
    |>  fetch_user
    |>? load_account
    |>? get_balance

log_attempt(report)
report
```

### `$` - Low-Precedence Application

`$` sits at precedence level -1, below all pipes. The pipeline on the right resolves
entirely before `$` applies the outermost function. `$` is right-associative.

```rs
echo $ raw_data |> parse_csv |> summarize

f $ g $ x
```

### Pipeline Examples

```rs
let report: string =
    raw_data
    |> parse_csv
    |> filter(valid_row)
    |> map(to_user)
    |> sort_by(u -> u.name)
    |> format_table

let result: Result<Profile, string> =
    request.body
    |>  parse_json
    |>? validate_schema
    |>? authenticate(token)
    |>? fetch_profile

echo $ users |> filter(active) |> map(u -> u.name) |> join(", ")
```
