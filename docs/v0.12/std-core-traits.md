# std::core::traits

This page documents the canonical standard traits currently assumed by the rest of the
spec.

## Draft Surface

```rust
module std::core::traits

def Display :: trait {
    show: Self -> string,
}

def Eq<T> :: trait {
    equals: T, T -> bool,
}

def Ord<T> :: trait {
    compare: T, T -> int,
}

def Hash<T> :: trait {
    hash: T -> int,
}

def Clone<T> :: trait {
    clone: T -> T,
}

def Add<T> :: trait {
    add: Self, T -> Self,
}

def Sub<T> :: trait {
    sub: Self, T -> Self,
}

def Mul<T> :: trait {
    mul: Self, T -> Self,
}

def Div<T> :: trait {
    div: Self, T -> Self,
}

def Rem<T> :: trait {
    rem: Self, T -> Self,
}

def Neg :: trait {
    neg: Self -> Self,
}

def BitNot :: trait {
    bit_not: Self -> Self,
}

def BitShiftLeft<T> :: trait {
    shift_left: Self, T -> Self,
}

def BitShiftRight<T> :: trait {
    shift_right: Self, T -> Self,
}

def Number :: trait {}

def Integer :: trait {}

def Signed :: trait {}

def Unsigned :: trait {}

def Float :: trait {}

def FixedWidth :: trait {
    bits: usize,
}

def Bounded :: trait {
    min: Self,
    max: Self,
}

def IndexLike :: trait {
    to_usize: Self -> usize,
}

def FromIntLiteral :: trait {
    from_int_literal: ComptimeInt -> Option<Self>,
}

def FromFloatLiteral :: trait {
    from_float_literal: ComptimeFloat -> Option<Self>,
}

def Functor<F :: Type -> Type> :: trait {
    map: <A, B> (A -> B), F<A> -> F<B>,
}

def Applicative<F :: Type -> Type> :: trait {
    pure: <A> A -> F<A>,
    apply: <A, B> F<(A -> B)>, F<A> -> F<B>,
}

def Monad<M :: Type -> Type> :: trait {
    pure: <A> A -> M<A>,
    bind: <A, B> M<A>, (A -> M<B>) -> M<B>,
}

def IntoFallible<T> :: trait {
    into_fallible: Self -> Fallible<T>,
}

def IndexDomain :: trait {
    to_offset: Self -> usize,
}

def FiniteIndex :: trait {
    count: usize,
}

def SequentialIndex :: trait {
    from_offset: usize -> Option<Self>,
}

def Index<I, T> :: trait {
    index: Self, I -> T,
}

def TryIndex<I, T> :: trait {
    get: Self, I -> Option<T>,
}
```

## Notes

- These traits are standard-library declarations, not parser keywords.
- `Display::show` produces user-facing text. It does not print. Output surfaces such as
  `echo` and `println` decide where text is written.
- The compiler still depends on some of them indirectly through sugar such as string
  interpolation or fallible piping.
- Numeric traits are capability traits. They describe available operations and properties
  instead of forming a runtime inheritance tree.
- `Number`, `Integer`, `Signed`, `Unsigned`, and `Float` classify numeric categories.
  Concrete primitives such as `i32`, `usize`, and `f64` provide the runtime layout and
  code generation behavior.
- `ComptimeInt` and `ComptimeFloat` are compiler-only literal domains. They are not
  ordinary runtime types.
- `IndexLike` means a value can be losslessly treated as a `usize` for indexing-related
  APIs. Not every unsigned type must implement it on every target.
- `IndexDomain`, `FiniteIndex`, and `SequentialIndex` classify types that may serve as
  collection index domains.
- `Index<I, T>` describes total indexing. `TryIndex<I, T>` describes partial indexing.
  The `[]` syntax is stricter than plain trait dispatch and is accepted only when total
  or proven safe.
- Future revisions should decide which of these are always available and which require
  import from a canonical module or prelude.
- Instance-style trait members are called through the trait namespace, such as
  `Display::show(value)` or `Monoid::combine(a, b)`.
- Associated trait values are accessed by type, such as `Monoid::empty<MyList>`.

## Related Pages

- [[traits-and-implementations]]
- [[generics-and-kinds]]
- [[std-error]]
