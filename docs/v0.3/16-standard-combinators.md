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

