# std::collections

This page documents the current collection direction for v0.12.

Fixed-size arrays such as `T[N]` and `T[N: I]` are language-level type expressions
documented in [[types-and-data-definitions]]. This page owns the standard dynamic
collection surface.

## Dynamic Arrays

`Arr<T>` is the default growable array type.

```rust
let values :: Arr<int>
```

`Arr<T>[I]` is a bounded growable array with custom index domain `I`.

```rust
def PersonId :: enum<usize> {
    John,
    Paul,
    Greg,
}

let queue :: Arr<string>[PersonId]
```

For `Arr<T>[I]`, the index domain `I` must be finite. If the collection supports push
allocation, the domain must also have a deterministic sequential ordering.

`Arr<T>[I]` is not a sparse map. It is a dense dynamic sequence whose currently occupied
indices form a prefix of the domain's declared order.

## Safe Indexing

`[]` must never panic. It is accepted only when indexing is total or when the compiler
can prove the index is valid for the specific collection value.

```rust
let a :: int[3]

let x := a[0]
```

```rust
if values.has_index(i) {
    let x := values[i]
}
```

```rust
for i in values.indices {
    echo values[i]
}
```

If the compiler cannot prove index validity, the program must use a partial access API.

```rust
let maybe_value := values.get(i)
```

For dynamic arrays, two facts are distinct:

- the index belongs to the declared domain
- the index is currently occupied

For that reason, `Arr<T>[I]` does not provide unconditional total indexing over all
values of `I`.

## Current Surface

The exact internal representation of collection types is not specified here. The current
surface is:

```rust
let len          :: Arr<T> -> usize
let capacity     :: Arr<T> -> usize
let get          :: Arr<T>, usize -> Option<T>
let has_index    :: Arr<T>, usize -> bool
let push         :: Arr<T>, T -> unit

let bounded_len       :: Arr<T>[I] -> usize
let bounded_capacity  :: Arr<T>[I] -> usize
let bounded_get       :: Arr<T>[I], I -> Option<T>
let bounded_has_index :: Arr<T>[I], I -> bool
let push              :: Arr<T>[I], T -> Fallible<unit>
let push_indexed      :: Arr<T>[I], T -> Fallible<I>
```

`indices` is part of the standard collection surface even though its exact proof-carrying
type is flow-sensitive. In `for i in arr.indices`, the loop binder `i` is treated as a
proven-valid index for that specific array value during the loop body.

## Related Pages

- [[types-and-data-definitions]]
- [[patterns-and-control-flow]]
- [[std-core-traits]]
- [[std-effects]]
