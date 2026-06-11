# std::effects

This page documents the standard effect declarations and handler surfaces currently
assumed by the rest of the spec.

## Draft Surface

```rust
module std::effects

def Yield<T> :: effect {
    @notation
    @position(prefix) @bind(right->value) @precedence(10)
    yield: T -> void,
}

def IO :: effect;

@notation
@position(prefix) @bind(right->body) @precedence(10)
@handles(Yield<T>)
let (stream): <T> (void -> void) -> Stream<T>

@notation
@position(prefix) @bind(right->body) @precedence(10)
@handles(Yield<T>)
let (collect): <T, C = Arr> (void -> void) -> C<T>

@notation
@position(prefix) @bind(right(1)->value, right(2)->arms) @precedence(10)
@handles(Repair)
let (recover): <T> Fallible<T>, RecoverArms -> Fallible<T>

@notation
@position(prefix) @bind(right->value) @precedence(10)
let (resume): <R> R -> Never
```

## Scope

- `Yield<T>`, `IO`, `stream`, `collect`, `recover`, and `resume` are documented here as
  standard surfaces.
- The meaning of effect declarations, handler scoping, and effect visibility remains owned
  by [[effects-and-handlers]].
- Future effect families such as `Await<T>`, `Receive<M>`, `Send<M>`, and `Spawn<M>` are
  still design-direction material rather than committed standard surfaces.

## Related Pages

- [[effects-and-handlers]]
- [[std-error]]
