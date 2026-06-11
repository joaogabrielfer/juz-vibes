# std::error

This page documents the standard carrier and family used for error propagation.

## Draft Surface

```rust
module std::error

def Fallible<T> :: sum {
    Ok(T),
    Err(Error),
}

def Error :: expandable sum with {
    meta: prod {
        location: Option<SourceLoc>,
        notes: Arr<string>,
        frames: Arr<ErrorFrame>,
    } = (location: .None, notes: [], frames: []),
}

def IntoFallible<T> :: trait {
    into_fallible: Self -> Fallible<T>,
}
```

## Propagation Surface

The language owns the syntax of:

- postfix `?`
- captured `?(name) { ... }`
- fallible pipe `|>?`

This page owns the standard declarations those forms target:

- `Fallible<T>`
- `Error`
- `IntoFallible<T>`

## Repairable Failure Boundary

Repair-response declarations and handled repair flow are documented in [[std-effects]]
because the recovery path depends on the effect system instead of plain sum-type data
alone.
