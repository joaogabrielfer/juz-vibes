## 17. Error Handling

Element provides two complementary error handling mechanisms with a clear design boundary
between functional and imperative styles.

### The `Fallible` Trait

`?` desugars through the `Fallible` trait, making error propagation customizable for
user-defined types.

```rs
def Fallible :: trait {
    is_err:  (Self) -> bool,
    unwrap:  (Self) -> Ok_T,
    err_val: (Self) -> Err_T,
}
```

### `?` - Three Forms

Postfix `?` unwraps on success or propagates the error upward:

```rs
let user:    User    = fetch_user(id)?
let profile: Profile = load_profile(user)?
let avatar:  Arr<u8> = fetch_avatar(profile.avatar_url)?
```

Infix `?` with a block handles the error in place:

```rs
let config := load_config("app.json")? {
    echo "config file missing; using defaults";
    default_config()
}
```

Typed error capture binds the error value:

```rs
let data := fetch_remote(url)?(e: NetworkError) {
    log_error("Network failure: {e.show}");
    .Ok(cached_fallback())
}
```

### Custom `Fallible`

```rs
def HttpResponse :: prod {
    status: int,
    body:   string,
}

@impl(Fallible)
def HttpResponse :: extends {
    is_err  = { self.status >= 400 },
    unwrap  = { self.body },
    err_val = { HttpError { code: self.status, message: self.body } },
}

let body := fetch(url)?
let body := fetch(url)? { "default response" }
let body := fetch(url)?(e: HttpError) {
    log("HTTP {e.code}: {e.message}");
    cached_response()
}
```

`Fallible` can model richer validation structures, not only first-error `Result` values:

```rs
def Validated<T> :: sum {
    Valid(T),
    Invalid(Arr<string>),
}

@impl(Fallible)
def Validated<T> :: extends {
    is_err = {
        match self [
            .Valid(_)   => false
            .Invalid(_) => true
        ]
    },

    unwrap = {
        match self [
            .Valid(v)   => v
            .Invalid(_) => panic "unwrap on Invalid"
        ]
    },

    err_val = {
        match self [
            .Invalid(errs) => errs
            .Valid(_)      => []
        ]
    },
}

let value := validate_email(input)?
let value := validate_email(input)?(errs: Arr<string>) {
    errs |> map(e -> "- " <> e) |> join("\n") |> println;
    default_value
}
```

### `|>?` vs `?` vs '>>='

| | `|>?` | `?` | '>>=' |
|---|---|---|---|
| Style | Functional, pipeline | Imperative, sequential | Functional, monadic |
| On error | Stays wrapped, chain continues | Unwinds frame, function returns early | Depends on monad implementation |
| Result type in binding | Wrapped fallible type | Unwrapped success type | Wrapped monadic type |
| Use when | Fallible pipeline propagation | Sequential steps that should abort on failure | General monad chaining via `std.functional` |

```rs
let report: Result<Report, string> =
    user_id
    |>  fetch_user
    |>? load_account
    |>? compute_report

log_attempt(report)
report

let user:    User    = fetch_user(user_id)?
let account: Account = load_account(user)?
let report:  Report  = compute_report(account)?

notify_success(user.id)
.Ok(report)

let report2 =
    get_user(user_id)
    >>= load_account
    >>= compute_report
```
