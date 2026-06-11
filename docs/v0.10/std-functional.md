# std.functional

`std.functional` provides import-only functional notations and helper elements.
Nothing in this module is compiler-special-cased.

## Draft Surface

```rust
module std.functional

@notation
let (>>): <A, B, C> (A -> B), (B -> C) -> (A -> C)

@notation
let (<<): <A, B, C> (B -> C), (A -> B) -> (A -> C)

@notation
let (>>=): <M :: Type -> Type, A, B> M<A>, (A -> M<B>) -> M<B>

@notation
let (<$>): <F :: Type -> Type, A, B> (A -> B), F<A> -> F<B>

@notation
let (<*>): <F :: Type -> Type, A, B> F<(A -> B)>, F<A> -> F<B>

@notation
let (<|>): <F> F, F -> F

@notation
let (&): <A, B> A, (A -> B) -> B

@notation
let (#): <A> int, (A -> A) -> (A -> A)

let id    :: <T> T                              -> T;
let const :: <A, B> A, B                        -> A;
let flip  :: <A, B, C> (A, B -> C)              -> (B, A -> C);
let on    :: <A, B, C> (B, B -> C), (A -> B)    -> (A, A -> C);
let fix   :: <A, B> ((A -> B), A -> B)          -> (A -> B);
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
