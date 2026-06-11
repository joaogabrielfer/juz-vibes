# std::core::types

This page starts the declaration-style documentation for common standard data types.

The list below is intentionally conservative. It documents surfaces already implied by the
current spec instead of inventing a full library catalog.

## Draft Surface

```rust
module std::core::types

def Option<T> :: sum {
    Some(T),
    None,
}

def Either<L, R> :: sum {
    Left(L),
    Right(R),
}

def Fallible<T> :: sum {
    Ok(T),
    Err(Error),
}

def SourceLoc :: prod {
    file: string,
    line: int,
    column: int,
}

def ErrorFrame :: prod {
    phase: string,
    message: string,
    location: Option<SourceLoc>,
}
```

## Scope

- `Fallible<T>` is listed here for discoverability, but its failure semantics are owned by
  [[std-error]].
- `SourceLoc` and `ErrorFrame` are support types for the error model and may eventually
  move under a dedicated error module namespace.
- Collection and container modules are still under-specified and are intentionally not
  expanded here yet.
- The primitive `void` type is documented in [[types-and-data-definitions]].

## Related Pages

- [[std-error]]
- [[types-and-data-definitions]]
