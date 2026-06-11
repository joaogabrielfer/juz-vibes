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

