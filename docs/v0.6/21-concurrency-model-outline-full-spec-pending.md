## 21. Concurrency Model *(Outline - Full Spec Pending)*

Element's planned concurrency model is based on isolated actors communicating via typed
mailboxes. Actors share no memory. All communication is message-passing via `send` and
`receive`.

```rs
use pid := spawn();

receive [
    ("ping", sender) => sender |> send("pong")
    ("echo", msg)    => echo msg
    ("stop")         => return;
    (..)             => echo "unknown message"
]

pid |> send("ping", self)
pid |> send("stop")
```

Coroutines with `yield` usable as lazy streams are planned. Full mailbox typing, actor
supervision trees, the general async model, and the interaction between the actor model
and `use` are pending specification.
