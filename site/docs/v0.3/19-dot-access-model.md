## 19. Dot Access Model

Dot notation in Element has exactly two meanings. It is **not** method-call syntax:

- `foo.field`    — direct field access on a `prod` type
- `foo.computed` — access a zero-argument computed property from `extend`,
                   with `self` implicitly bound to `foo`

**There are no method calls.** Functions requiring arguments are invoked via pipes or
direct application:

```rs
// NOT valid:
user.format("json")         // no method calls with arguments

// Valid alternatives:
format(user, "json")
user |> format("json")
format_json(user)
```

### Computed Properties via `extend`

Computed properties follow the thunk model — evaluated on first access, cached:

```rs
def @impl(Display, Hash) User = extend {
    self: prod { id: int, name: string, email: string };

    // Computed properties — zero-argument, cached after first access:
    show:     string = { format("User({})", self.name) };
    domain:   string = { self.email |> split("@") |> last };
    initials: string = { self.name |> words |> map(w -> w |> first_char) |> join("") };
    hash:     u64    = { hash_combine(self.id.hash, self.name.hash) };
};

let u := User(1, "Alice Smith", "alice@example.com");
echo u.show;      // "User(Alice Smith)"
echo u.domain;    // "example.com"  — cached
echo u.initials;  // "AS"           — cached
echo u.hash;      // computed once
```

### The `Time` Type — Canonical Example

```rs
def @impl(Display, Eq, Ord<Time>) Time = extend {
    self: int;   // unix timestamp in seconds — the only stored value

    ms:      int = { self * 1_000 };
    us:      int = { self * 1_000_000 };
    ns:      int = { self * 1_000_000_000 };
    s:       int = { self };
    minutes: int = { self / 60 };
    hours:   int = { self / 3_600 };
    days:    int = { self / 86_400 };

    show:    string = { format("{}s ({}ms)", self.s, self.ms) };
    equals   = (a, b) -> a.self == b.self;
    compare  = (a, b) -> a.self - b.self;
};

let t := now();
echo t.ms;       // no method call — reads like a field, computes once
echo t.days;     // same
echo t.show;     // "1718000000s (1718000000000ms)"
```

---

