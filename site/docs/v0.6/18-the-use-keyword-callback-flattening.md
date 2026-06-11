## 18. The use Keyword - Callback Flattening

`use` replaces `let` in a binding to signal that the right-hand function takes a callback
as its final argument. `use` captures the remainder of the current block as an implicit
closure passed to that final argument, eliminating callback nesting.

```rs
database.connect(config, conn -> {
    conn.begin_transaction(session -> {
        open("import.csv", file -> {
            file.read() |> parse_csv |> insert_rows(session)
        })
    })
})
```

With `use`:

```rs
use conn    := database.connect(config);
use session := conn.begin_transaction();
use file    := open("import.csv");

file.read() |> parse_csv |> insert_rows(session)
```

The desugaring of `use conn := f(args)` is:

```rs
f(args, conn -> { /* rest of block */ })
```

### Without a Binding

```rs
use acquire_lock(mutex);
critical_section()
```

Desugars to:

```rs
acquire_lock(mutex, () -> { critical_section() })
```

### Resource Management

The called function handles cleanup after the callback returns.

```rs
use file := open("data.txt");
let content := file.read();
content |> process |> echo
```

### Actor Spawning

```rs
use pid := spawn();
receive [
    ("ping", sender) => sender |> send("pong")
    ("stop")         => return;
    (..)             => echo "unknown message"
]
```

### Multiple `use` Bindings

Multiple `use` bindings stack naturally, with each binding capturing the rest of the
block.

```rs
use conn    := database.connect(config);
use session := conn.begin_transaction();
use lock    := acquire_write_lock(resource);

do_work(session, lock)
```

The exact scope where `be` should appear inside `use` continuation blocks is open. See
[[open-questions]].
