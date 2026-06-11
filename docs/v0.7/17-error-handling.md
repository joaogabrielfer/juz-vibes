## 17. Error Handling

Element provides two complementary error handling mechanisms with a clear design boundary
between functional and imperative styles.

### The Standard Error Carrier: `Fallible<T>`

Idiomatic error propagation uses:

```rs
def Fallible<T> :: sum {
    Ok(T),
    Err(Error),
}
```

`Fallible<T>` always stores `Error` on the failure side.

For non-standard two-sided data flows, use `Either<L, R>` or a custom sum.

### The `IntoFallible<T>` Trait

`?` desugars through `IntoFallible<T>`, making propagation customizable for user-defined
carrier types.

```rs
def IntoFallible<T> :: trait {
    into_fallible: (Self) -> Fallible<T>,
}
```

Canonical implementation:

```rs
@impl(IntoFallible<T>)
def Fallible<T> :: extends {
    into_fallible = { self },
}
```

### `?` - Two Forms

Postfix `?` unwraps on success or propagates the error upward:

```rs
let user:    User    = fetch_user(id)?
let profile: Profile = load_profile(user)?
let avatar:  Arr<u8> = fetch_avatar(profile.avatar_url)?
```

Capture `?` with an explicit binder handles the error in place:

```rs
let config := load_config("app.json")?(err) {
    echo "config failed: {err}";
    default_config()
}
```

The binder name is arbitrary. Typed capture syntax is not part of v0.7.

### Expandable `Error` Family

```rs
def SourceLoc :: prod { file: string, line: int, column: int }

def ErrorFrame :: prod {
    phase: string,
    message: string,
    location: Option<SourceLoc>,
}

@requires(Display)
def Error :: expandable sum with {
    meta: prod {
        location: Option<SourceLoc>,
        notes: Arr<string>,
        frames: Arr<ErrorFrame>,
    } = (location: .None, notes: [], frames: []),
}

@impl(Display)
def Error.Generic :: extends {
    self: string,
    show = { self },
}

@impl(Display)
def Error.Http :: extends {
    self: prod { code: int, message: string },
    show = { "http {self.code}: {self.message}" },
}
```

`Error.*` members are concrete nested types. Their constructors produce `Error` values:

```rs
let e1: Error = Error.Generic("bad state")
let e2: Error = Error.Http(code: 404, message: "not found")
let e3: Error = Error.Http(
    code: 500,
    message: "internal",
    meta: (
        location: .Some(SourceLoc("server.el", 18, 3)),
        notes: ["during fetch profile"],
        frames: [],
    ),
)
```

Expandable-root pattern matching must include `_`:

```rs
match e2 [
    .Http(h)  => echo h
    .Generic(g) => echo g
    _ => echo "unknown error member"
]
```

### Custom `IntoFallible<T>`

```rs
def HttpResponse :: prod {
    status: int,
    body: string,
}

@impl(IntoFallible<string>)
def HttpResponse :: extends {
    into_fallible = {
        if self.status >= 400 =>
            .Err(Error.Http(code: self.status, message: self.body))
        else =>
            .Ok(self.body)
    },
}

let body := fetch(url)?
let body := fetch(url)?(problem) {
    echo "HTTP request failed: {problem}";
    cached_response()
}
```

### `|>?` vs `?` vs '>>='

| | `|>?` | `?` | '>>=' |
|---|---|---|---|
| Style | Functional, pipeline | Imperative, sequential | Functional, monadic |
| On error | Stays wrapped, chain continues | Unwinds frame, function returns early | Depends on monad implementation |
| Result type in binding | Wrapped fallible type | Unwrapped success type | Wrapped monadic type |
| Use when | Pipeline propagation through `IntoFallible` steps | Sequential steps that should abort on failure | General monad chaining via `std.functional` |

```rs
let report: Fallible<Report> =
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
