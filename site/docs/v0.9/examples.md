# Examples

This page is a quick entry point, not the canonical spec. Each example points back to the
reference chapter that owns the rule.

## Functions and Pipelines

```rust
let report :=
    request.body
    |> parse_json
    |>? validate_schema
    |>? authenticate(token)
    |>? fetch_profile
```

See:

- [[expressions-and-operators]]
- [[std-error]]

## Pattern Dispatch

```rust
let fib: (int) -> int = [
    (0) => 0
    (1) => 1
    (n: int) => recurse(n - 1) + recurse(n - 2)
]
```

See:

- [[patterns-and-control-flow]]

## Traits and Structural Subelements

```rust
def Time :: extends {
    self: prod {
        seconds: int,
        ms: int = self.seconds * 1_000,
        show: string = "{self.ms}ms",
    },

    @impl(Display) {
        show = self.show
    }
}
```

See:

- [[traits-and-implementations]]
- [[evaluation-model]]

## Handled Yield

```rust
let squares := collect {
    for n in range(0, 10) {
        yield n * n
    }
}
```

See:

- [[effects-and-handlers]]
- [[std-effects]]
