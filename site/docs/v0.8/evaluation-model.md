# Evaluation Model

This chapter owns lazy access, cached thunks, computed subelements, dot access, and
callback flattening.

## Zero-Argument Elements

Every zero-argument binding is a managed thunk. Two evaluation modes govern when the body
is executed.

## Default First-Access Caching

A pure zero-argument element evaluates its body exactly once, on first bare access in its
scope. Subsequent bare accesses return the cached value. Appending `()` forces fresh
re-evaluation and bypasses the cache.

```rust
let @pub timestamp: int = get_unix_time()

let t1 := timestamp    // body evaluated here, result cached
let t2 := timestamp    // cache hit, same value as t1
let t3 := timestamp()  // forces fresh evaluation
```

`foo` accesses the cached value. `foo()` calls the element again.

## `@lazy`

`@lazy` removes the internal cache entirely. Every bare access triggers fresh execution.
For a `@lazy` element, `foo` and `foo()` are equivalent.

```rust
let @pub @lazy live_clock: int = get_unix_time()

let c1 := live_clock
let c2 := live_clock
let c3 := live_clock()
```

## Effectful Zero-Argument Elements

Pure zero-argument elements are lazy and cached by default. Effectful zero-argument
elements are lazy but must not be implicitly cached. Each bare access evaluates the body
again unless a later explicit effect handler defines different behavior.

```rust
@effects(IO)
let live_clock: Time = current_time()

let c1 := live_clock // runs current_time()
let c2 := live_clock // runs current_time() again
```

`@memoize` requires `@pure`. A declaration that performs `IO`, `Yield<T>`,
`Repair<Error.X>`, or any visible effect cannot be memoized unless the effect is fully
handled inside the declaration and the public declaration is checked as pure.

## Dot Access

Dot notation has exactly two meanings:

- `foo.name`: direct stored subelement access on a structural type
- `foo.computed`: access a zero-argument computed subelement from an implementation, with
  `self` implicitly bound to `foo`

Dot access is not method-call syntax.

```rust
// Invalid:
user.format("json")

// Valid:
format(user, "json")
user |> format("json")
format_json(user)
```

Attached subelements declared with `sum ... with {}` are stored subelements and use the
same direct access model.

## Computed Properties

Computed properties are zero-argument elements associated with a type implementation.
They are accessed with dot notation and cached according to the same rules as any other
thunk.

```rust
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

Trait subelements such as `show` and `hash` do not need type annotations inside `extends`
when their types are known from the trait. Computed subelements not declared by a trait
may specify their type explicitly.

## Canonical Time Example

```rust
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

## `use`

`use` replaces `let` in a binding to signal that the right-hand function takes a callback
as its final argument. `use` captures the remainder of the current block as an implicit
closure passed to that final argument.

```rust
database.connect(config, conn -> {
    conn.begin_transaction(session -> {
        open("import.csv", file -> {
            file.read() |> parse_csv |> insert_rows(session)
        })
    })
})
```

With `use`:

```rust
use conn    := database.connect(config);
use session := conn.begin_transaction();
use file    := open("import.csv");

file.read() |> parse_csv |> insert_rows(session)
```

The desugaring of `use conn := f(args)` is:

```rust
f(args, conn -> { /* rest of block */ })
```

Without a binding:

```rust
use acquire_lock(mutex);
critical_section()
```

Desugars to:

```rust
acquire_lock(mutex, () -> { critical_section() })
```

Multiple `use` bindings stack naturally, with each binding capturing the rest of the
block.

```rust
use conn    := database.connect(config);
use session := conn.begin_transaction();
use lock    := acquire_write_lock(resource);

do_work(session, lock)
```

The exact scope where `be` should appear inside `use` continuation blocks remains open in
[[open-questions]].
