## 19. Dot Access Model

Dot notation in Element has exactly two meanings. It is not method-call syntax.

- `foo.field` - direct field access on a `prod` type
- `foo.computed` - access a zero-argument computed property from an implementation, with
  `self` implicitly bound to `foo`

Functions requiring arguments are invoked through normal function calls or pipes.

```rs
// Invalid:
user.format("json")

// Valid:
format(user, "json")
user |> format("json")
format_json(user)
```

### Computed Properties via `extends`

Computed properties follow the thunk model. They are evaluated on first access and then
cached unless marked `@lazy`.

```rs
@impl(Display, Hash)
def User :: extends {
    self: prod { id: int, name: string, email: string },

    show = { "User({self.name})" },
    domain: string = { self.email |> split("@") |> last },
    initials: string = {
        self.name |> words |> map(w -> w |> first_char) |> join("")
    },
    hash = { hash_combine(self.id.hash, self.name.hash) },
}

let u := User(1, "Alice Smith", "alice@example.com")
echo u.show
echo u.domain
echo u.initials
echo u.hash
```

Trait fields such as `show` and `hash` do not need type annotations inside `extends`
when their types are known from the trait. Computed properties that are not declared by a
trait may specify their type explicitly.

### Canonical Time Example

```rs
@impl(Display, Eq, Ord<Time>)
def Time :: extends {
    self: int,

    ms:      int = { self * 1_000 },
    us:      int = { self * 1_000_000 },
    ns:      int = { self * 1_000_000_000 },
    s:       int = { self },
    minutes: int = { self / 60 },
    hours:   int = { self / 3_600 },
    days:    int = { self / 86_400 },

    show = { "{self.s}s ({self.ms}ms)" },
    equals = (a, b) -> a.self == b.self,
    compare = (a, b) -> a.self - b.self,
}

let t := now()
echo t.ms
echo t.days
echo t.show
```
