# std.functional

`std.functional` provides import-only functional notations and helper elements.
Nothing in this module is compiler-special-cased.

## Draft Surface

```rust
module std.functional

@notation
let (>>): <A, B, C> (def((A) -> B), def((B) -> C)) -> def((A) -> C)

@notation
let (<<): <A, B, C> (def((B) -> C), def((A) -> B)) -> def((A) -> C)

@notation
let (>>=): <M :: Type -> Type, A, B> (M<A>, def((A) -> M<B>)) -> M<B>

@notation
let (<$>): <F :: Type -> Type, A, B> (def((A) -> B), F<A>) -> F<B>

@notation
let (<*>): <F :: Type -> Type, A, B> (F<def((A) -> B)>, F<A>) -> F<B>

@notation
let (<|>): <F> (F, F) -> F

@notation
let (&): <A, B> (A, def((A) -> B)) -> B

@notation
let (#): <A> (int, def((A) -> A)) -> def((A) -> A)

let id: <T> (T) -> T
let const: <A, B> (A, B) -> A
let flip: <A, B, C> (def((A, B) -> C)) -> def((B, A) -> C)
let on: <A, B, C> (def((B, B) -> C), def((A) -> B)) -> def((A, A) -> C)
let fix: <A, B> (def((def((A) -> B), A) -> B)) -> def((A) -> B)
```

## Import Style

```rust
import std.functional
import std.functional { (>>), (>>=), (<$>), flip, on, id, fix }
```

## Notes

- `|>?` remains part of the core pipe family and is not defined here.
- `>>=` remains the general imported monadic bind form.
- This module is the main example of why the docs need a distinct standard-library area.
