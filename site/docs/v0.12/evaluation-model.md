# Evaluation Model

This chapter owns zero-argument evaluation, initialization order for self-derived
defaults, dot access, and `with` callback flattening.

## Zero-Argument Elements

Every zero-argument element is demand-driven, but value-like declarations and callable
zero-argument declarations are distinct.

- `let name: T = expr` declares a value-like zero-argument element.
- `let name: unit -> T = expr` or `let name :: unit -> T` declares a callable
  zero-argument element.

Value-like zero-argument elements are accessed as `name`. Callable zero-argument
elements are invoked as `name()`. A callable zero-argument element is not implicitly
evaluated by bare access and does not receive first-access caching.

## Default First-Access Caching

A pure zero-argument element evaluates its body on first demand in its scope. Subsequent
bare accesses may reuse the stored result. This rule applies only to value-like
declarations.

```rust
let @pub timestamp: int = get_unix_time()

let t1 := timestamp    // body evaluated here, result cached
let t2 := timestamp    // cache hit, same value as t1
```

`foo` performs ordinary access. A value-like zero-argument element is not called with
`()`.

Fresh evaluation belongs to callable zero-argument elements:

```rust
let now: unit -> Time = current_time()

let t1 := now()
let t2 := now()
```

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
`Repair<Error::X>`, or any visible effect cannot be memoized unless the effect is fully
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

def User :: extends {
	@impl(Display) {
		show = self.show	
	}
}

let u := User(1, "Alice Smith", "alice@example.com")
echo u.domain;
echo u.initials;
echo u; // calls Display::show(u)
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
        mut seconds: int, //let only the 'root' subelement be mutable for now
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
echo t.ms;
echo t.days;
echo t;
```

## `with`

`with` creates a scope-attached binding for a function that receives a callback as its
final argument. The binding remains active until the end of the current lexical block
unless it is ended earlier with `end name;`.

A `with`-compatible function must let the callback determine the scoped computation
result. The function may return that result directly or wrap it in an explicit carrier
such as `Fallible<R>`.

```rust
let open        :: Path, (File -> R) -> R
let open_result :: Path, (File -> R) -> Fallible<R>
```

```rust
database::connect(config, conn -> {
    begin_transaction(conn, session -> {
        open("import.csv", file -> {
            file |> read |> parse_csv |> insert_rows(session)
        })
    })
})
```

With `with`:

```rust
{
    let connection_id := with conn <- database::connect(config);
    with session <- begin_transaction(conn);
    with file <- open("import.csv");

    file |> read |> parse_csv |> insert_rows(session)
}
```

The active `with` bindings are closed in reverse creation order at the end of the block:
`file`, then `session`, then `conn`.

The conceptual desugaring of `with conn <- f(args);` is:

```rust
f(args, conn -> { /* rest of block */ })
```

If the scoped computation produces a value, the `with` expression has the callback result
or the wrapper returned by the callback-taking function.

```rust
let content := with file <- open("config.txt") {
    file |> read_to_string
}
```

APIs that need to expose additional scoped values should bind them through the callback
arguments rather than creating a second hidden return channel.

```rust
let content := with file, meta <- open_with_meta("config.txt") {
    validate(meta);
    file |> read_to_string
}
```

Multiple `with` bindings stack naturally, with each binding capturing the continuation
that follows it in the same lexical block.

```rust
{
    with conn <- database::connect(config);
    with session <- begin_transaction(conn);
    with lock <- acquire_write_lock(resource);

    do_work(session, lock)
}
```

This corresponds to callback nesting:

```rust
database::connect(config, conn -> {
    begin_transaction(conn, session -> {
        acquire_write_lock(resource, lock -> {
            do_work(session, lock)
        })
    })
})
```

### Early `end`

`end name;` closes the active `with` binding named `name` and every active `with` binding
opened after it in the same continuous lexical scope.

```rust
{
    let connection_id := with conn <- database::connect(config);
    with session <- begin_transaction(conn);
    with file <- open("import.csv");

    file |> read |> parse_csv |> insert_rows(session);

    end conn;

    let file := "archive.csv"
}
```

`end conn;` closes `file`, `session`, and `conn`, in that order. After the `end`, those
bindings are out of scope and their names may be reused.

`end name;` cannot target a `with` binding from an outer lexical block.

```rust
{
    with conn <- database::connect(config);

    {
        with file <- open("import.csv");
        end conn; // invalid
    }
}
```

A `with` bind name is an ordinary lexical binding while active. A scope cannot declare
another binding with the same name until the active `with` has ended.

```rust
{
    with file <- open("a.csv");
    let file := "b.csv"; // invalid
}
```
