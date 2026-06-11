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
a |> bar          // bar(a)

users |> filter(active)
value |>> divide(100.0)
item  |>>> insert(db, "users_table")
```

### Placeholder Pipe `|~`

`~` marks exactly where the piped value or values land. The count of `~` must precisely
match the number of values being piped. Use `|~` when the insertion point is not the
default positional slot.

```rs
x, y    |~ f(~, ~, z, 4)
x, y, z |~ f(0, ~, ~, ~)
value   |~ transform(pre, ~, post)

// Compile error: 2 piped values but 3 placeholders.
x, y |~ f(~, ~, ~, 4)
```

### Value Group Spreading

```rs
let coords := (10, 20)

coords   |> draw_at(style)    // draw_at(coords, style)
coords.. |> draw_at(style)    // draw_at(10, 20, style)
```

### Fallible Pipeline `|>?`

`|>?` is the fallible variant of the pipe family and shares precedence level 0 with other
pipe operators. It applies only when the left-hand type implements `IntoFallible`.

If an error or failure variant is encountered, it stays wrapped and propagates through the
rest of the chain. The outer function continues executing with the wrapped result.

```rs
let report := user_id
    |>  fetch_user
    |>? load_account
    |>? get_balance

log_attempt(report)
report
```

### Reverse Application `$`, `$$`, `$$$`

`$` sits at precedence level -1, below all pipes. `$` is right-associative and mirrors the
positional behavior of the pipe family:

- `f $ x` means `f(x)`
- `f(a) $$ x` means `f(a, x)`
- `f(a, b) $$$ x` means `f(a, b, x)`

```rs
echo $ raw_data |> parse_csv |> summarize
divide(100.0) $$ value
insert(db, "users_table") $$$ item
```

`foo $$ bar` is invalid when the left side does not already provide an insertion slot for
the requested argument position.

### Placeholder Reverse Application `$~`

`$~` is reverse application with explicit `~` placeholders:

```rs
format $~ "Hello, ~!" $ get_name()
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

let result: Fallible<Profile> =
    request.body
    |>  parse_json
    |>? validate_schema
    |>? authenticate(token)
    |>? fetch_profile

echo $ users |> filter(active) |> map(u -> u.name) |> join(", ")
```
