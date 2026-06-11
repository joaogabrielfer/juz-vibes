# Effects and Handlers

Effects are first-class language elements used to describe operations that may suspend,
interact with the runtime, request repair, or be interpreted by a handler.

The language core owns effect declaration syntax, visible effect sets, handler checking,
purity checking, and cleanup obligations. Standard effect declarations and handler names
are documented in [[std-effects]].

## Effect Declarations

Effects are declared with `def ... :: effect`.

```rust
def Yield<T> :: effect {
    @notation
    @position(prefix) @bind(right->value) @precedence(10)
    yield: (T) -> (),
}
```

An effect operation has a function-like type, but performing the operation may be handled
by an enclosing handler instead of running like an ordinary function call.

Effect operations may declare notation. Core effect notation such as `yield value` is
available by default through the prelude. User-defined effect notation must be explicitly
imported before it may introduce keyword-like syntax into another module.

## Declaring Effects on Functions

`@effects(...)` declares the visible effects a function may perform.

```rust
@effects(Yield<int>, IO)
let read_numbers: (Path) -> Fallible<()> = path -> {
    use file := open(path);

    for line in file.lines {
        yield parse_int(line)?
    }

    .Ok(())
}
```

Rules:

- local and private functions may infer effects
- public functions should declare visible effects
- `@pure` declares and checks an empty effect set
- effects performed and fully handled inside a function do not appear in its public effect
  set

## Handlers

`@handles(...)` declares that a function or handler form consumes an effect from a body.

```rust
@notation
@position(prefix) @bind(right->body) @precedence(10)
@handles(Yield<T>)
let (collect): <T, C = Arr> (def(() -> ())) -> C<T>
```

The body parameter is a thunk or block that may perform the handled effect. The handler
defines the interpretation of that effect.

Core handler forms may use block operands. General user-defined block handler syntax is
deferred; `stream`, `collect`, and `recover` are the initial built-in/default handler
forms.

## Generators, Streams, and Collections

The old list-comprehension design is retired. Use `collect` for eager collections and
`stream` for lazy streams.

```rust
let squares := collect {
    for n in range(0, 10) {
        yield n * n
    }
}
```

`collect<C>` selects an explicit collection constructor. If omitted, `Arr` is inferred.

```rust
let unique_names := collect<Set> {
    for user in users {
        yield user.name
    }
}
```

`stream` handles `Yield<T>` lazily. The stream body must not execute until the stream is
pulled.

```rust
let naturals := stream {
    let mut n := 0;

    loop {
        yield n;
        n = n + 1;
    }
}
```

Advancing a stream is stateful and must use explicit call syntax.

```rust
let first := lazy.next()
```

Bare `lazy.next` must not advance a stream through cached thunk semantics.

## IO

`IO` marks interaction with the external runtime world: filesystem, terminal, network,
clock, process execution, and other runtime capabilities.

Rules:

- pure zero-argument elements are lazy and cached
- effectful zero-argument elements are lazy and uncached
- `@memoize` requires `@pure`
- compile-time code cannot perform `IO` unless an explicit compiler or build policy
  permits it
- tests may provide mock handlers or runtime capabilities for `IO`

## Recoverable Errors

`Fallible<T>` remains plain data. An `.Err` value never stores a live continuation.

Recoverable errors attach a repair response type to an `Error` member.

```rust
def ConfigRepair :: sum {
    UsePath(Path),
    UseText(string),
}

@recoverable(ConfigRepair)
def Error.FileMissing :: prod {
    path: Path,
}
```

The callee explicitly decides where repair is possible with `repair e`.

```rust
@effects(IO, Repair<Error.FileMissing>)
let read_config_text: (Path) -> Fallible<string> = path -> {
    match read_file(path) [
        .Ok(text) => .Ok(text)

        .Err(e: Error.FileMissing) => {
            match repair e [
                .Some(.UsePath(next)) => read_config_text(next)
                .Some(.UseText(text)) => .Ok(text)
                .None                => .Err(e)
            ]
        }

        .Err(e) => .Err(e)
    ]
}
```

The caller handles repair opportunities with `recover`.

```rust
let result := recover load_config(path) [
    .FileMissing(e) => resume .UsePath(default_config_path)
]
```

Rules:

- `repair e` returns `Option<R>`, where `R` is the repair response type declared by
  `@recoverable(R)`
- `.None` means no active handler repaired the error
- `resume value` is valid only inside `recover` arms
- resumptions are one-shot in this revision
- matching a non-recoverable `Error` member in a `recover` arm is a compile error

## Recovery in Pipelines

Prefer placing `recover` immediately after the computation it repairs.

```rust
let config :=
    (path
    |> resolve_config_path
    |> load_config
    `recover` [
        .FileMissing(e) => resume .UsePath(default_config_path)
    ])?
    |> validate_config
```

When recovery is passed as an argument, use `$` to keep grouping clear.

```rust
let config :=
    validate_config $
    (load_config(path) `recover` [
        .FileMissing(e) => resume .UseText(default_config_text)
    ])?
```

## Async and Actor Direction

The effect system is intended to support the later async and actor model. The scheduler
can be modeled as a handler for suspension effects.

```rust
def Await<T> :: effect {
    @notation
    @position(prefix) @bind(right->task) @precedence(10)
    await: (Task<T>) -> T,
}

def Receive<M> :: effect {
    receive: () -> M,
}

def Send<M> :: effect {
    send: (Pid<M>, M) -> (),
}
```

`Receive<M>` and `Send<M>` should be mailbox-specific rather than actor-global. Actors are
runtime entities; mailboxes are typed communication surfaces.

## Cleanup and Cancellation

Suspended computations own their cleanup obligations.

Rules:

- dropping a stream must run cleanup for resources captured by the stream body
- unrepaired `repair e` returns `.None` and cleanup continues through ordinary control
  flow
- future async cancellation must run cleanup before the task is considered dead
- one-shot resumptions are required in this revision to keep cleanup and mutable state
  tractable

## Deferred Features

- multi-shot resumptions
- full async/await semantics
- actor supervision trees
- session-typed mailboxes
- general user-defined block handler syntax beyond core handler forms
- refinement types with `where`
- typestate declarations for resources and protocols
