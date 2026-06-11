## 18. The use Keyword — Callback Flattening

`use` replaces `let` in a binding to signal that the right-hand function takes a
**callback as its final argument**. `use` captures the entire remainder of the current
block as an implicit closure passed to that final argument, eliminating callback nesting.

```rs
// Without use — deeply nested:
database.connect(config, conn -> {
    conn.begin_transaction(session -> {
        open("import.csv", file -> {
            file.read() |> parse_csv |> insert_rows(session)
        })
    })
});

// With use — flat:
use conn    := database.connect(config);
use session := conn.begin_transaction();
use file    := open("import.csv");

file.read() |> parse_csv |> insert_rows(session);
```

The desugaring of `use conn := f(args)` is:
```rs
f(args, conn -> { /* rest of block */ })
```

**Without a binding** (when the yielded value is not needed):

```rs
use acquire_lock(mutex);
critical_section();
// desugars to: acquire_lock(mutex, () -> { critical_section() })
```

**Resource management** — the called function handles cleanup after the callback:

```rs
use file := open("data.txt");
let content := file.read();
content |> process |> echo;
// open() closes file after callback returns
```

**Actor spawning:**

```rs
use pid := spawn();
receive [
    ("ping", sender)  => sender |> send("pong")
    ("stop")          => return
    (..)              => echo "unknown message"
];
```

**Multiple `use` bindings** stack naturally, with each capturing the rest of the block:

```rs
use conn    := database.connect(config);
use session := conn.begin_transaction();
use lock    := acquire_write_lock(resource);

// All three resources active here, all cleaned up in reverse order
do_work(session, lock);
```

---

