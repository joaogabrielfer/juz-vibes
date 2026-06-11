## 21. Concurrency Model *(Outline — Full Spec Pending)*

Element's concurrency model is based on isolated actors communicating via typed mailboxes.
Actors share no memory. All communication is message-passing via `send` and `receive`.

```rs
// Spawning an actor with use:
use pid := spawn();
receive [
    ("ping", sender) => sender |> send("pong")
    ("echo", msg)    => echo msg
    ("stop")         => return
    (..)             => echo "unknown message"
];

// Sending messages:
pid |> send("ping", self);
pid |> send("stop");
```

Coroutines with `yield` usable as lazy streams are planned. Full mailbox typing, actor
supervision trees, and the interaction between the actor model and `use` are pending
specification.

---

