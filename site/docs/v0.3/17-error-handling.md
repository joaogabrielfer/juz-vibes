## 17. Error Handling

Element provides two complementary error handling mechanisms with a clear design boundary
between functional and imperative styles.

### `?` — Three Forms

`?` is an operator defined with `@position(postfix)`. All three forms desugar to function
calls, making them user-overridable for custom types.

**Postfix — propagate error upward, exit current function:**

```rs
let user:    User    = fetch_user(id)?;
let profile: Profile = load_profile(user)?;
let avatar:  Arr<u8> = fetch_avatar(profile.avatar_url)?;
// If any step returns Err, the function returns that Err immediately
```

**Infix with block — handle error in place, no binding:**

```rs
let config := load_config("app.json")? {
    echo "Config file missing — using defaults";
    default_config()
};
```

**Infix with typed error capture:**

```rs
let data := fetch_remote(url)?(e: NetworkError) {
    log_error(format("Network failure: {}", e.show));
    .Ok(cached_fallback())
};
```

### `|>?` vs `?` — Design Boundary

| | `\|>?` | `?` |
|---|---|---|
| Style | Functional, pipeline | Imperative, sequential |
| On error | Stays wrapped, chain continues | Unwinds frame, function returns early |
| Result type in binding | `Result<T, E>` (wrapped) | `T` (unwrapped value) |
| Use when | Composing fallible operations | Sequential steps where failure should abort |

```rs
// Functional — result stays wrapped throughout:
let report: Result<Report, string> =
    user_id
    |>  fetch_user
    |>? load_account
    |>? compute_report;

log_attempt(report);    // always runs
report                  // returns Result

// Imperative — each step unwraps or the function returns early:
let user:    User    = fetch_user(user_id)?;
let account: Account = load_account(user)?;
let report:  Report  = compute_report(account)?;

notify_success(user.id);   // only runs if all steps succeeded
.Ok(report)
```

---

