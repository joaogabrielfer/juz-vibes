# Evaluation Model

This chapter owns zero-argument evaluation, initialization order for self-derived
defaults, dot access, and callback flattening.

## Zero-Argument Elements

Every zero-argument binding is demand-driven. Evaluation rules depend on purity and on
whether the declaration is a top-level binding or a structural subelement default.

## Default First-Access Caching

A pure zero-argument element evaluates its body on first demand in its scope. Subsequent
bare accesses may reuse the stored result. Appending `()` forces fresh re-evaluation.

```rust
let @pub timestamp: int = get_unix_time()

let t1 := timestamp    // body evaluated here, result cached
let t2 := timestamp    // cache hit, same value as t1
let t3 := timestamp()  // forces fresh evaluation
```

`foo` performs ordinary access. `foo()` requests a fresh evaluation.

## Effectful Zero-Argument Elements

Effectful zero-argument elements are still demand-driven, but they must not be implicitly
cached. Each bare access evaluates the body again unless a later explicit handler or
library construct defines different behavior.

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

Dot notation resolves only structural subelements declared by the type.

- `foo.name`: direct stored subelement access on a structural type
- `foo.defaulted`: access a structural subelement whose initial value was derived during
  construction

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

Trait members are not resolved through dot access.

```rust
def User :: prod {
    id: int,
    name: string,
    email: string,
    domain: string = self.email |> split("@") |> last,
    initials: string = self.name |> words |> map(w -> w |> first_char) |> join(""),
    show: string = "User({self.name})",
}

let u := User(1, "Alice Smith", "alice@example.com")
echo u.domain
echo u.initials
echo Display.show(u)
```

## Self-Derived Defaults

A structural subelement default may reference `self`. Such defaults are evaluated during
construction and then stored in the instance. They are not recomputed on later dot
access.

Self-derived defaults must be pure. They must not perform visible effects such as `IO`,
`Yield<T>`, or mutation of external state.

Default initialization proceeds in declaration order. A default may reference only
subelements that are already initialized. Forward references and cycles are compile
errors.

```rust
def Point :: prod {
    x: int,
    y: int,
    show: string = "{self.x}, {self.y}",
}
```

This is valid because `show` depends only on `x` and `y`, which are already initialized.

```rust
def Bad :: prod {
    a: int = self.b,
    b: int = self.a,
}
```

This is a compile error because the defaults are cyclic.

## Canonical Time Example

```rust
def Time :: extends {
    self: prod {
        seconds: int,
        ms: int = self.seconds * 1_000,
        us: int = self.seconds * 1_000_000,
        ns: int = self.seconds * 1_000_000_000,
        minutes: int = self.seconds / 60,
        hours: int = self.seconds / 3_600,
        days: int = self.seconds / 86_400,
        show: string = "{self.seconds}s ({self.ms}ms)",
    },

    @impl(Display) {
        show = self.show
    }

    @impl(Eq<Time>) {
        equals = (a, b) -> a.seconds == b.seconds
    }

    @impl(Ord<Time>) {
        compare = (a, b) -> a.seconds - b.seconds
    }
}

let t := now()
echo t.ms
echo t.days
echo Display.show(t)
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
