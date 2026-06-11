# std.core.traits

This page documents the canonical standard traits currently assumed by the rest of the
spec.

## Draft Surface

```rust
module std.core.traits

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
```

## Notes

- These traits are standard-library declarations, not parser keywords.
- The compiler still depends on some of them indirectly through sugar such as string
  interpolation or fallible piping.
- Future revisions should decide which of these are always available and which require
  import from a canonical module or prelude.
- Instance-style trait members are called through the trait namespace, such as
  `Display.show(value)` or `Monoid.combine(a, b)`.
- Associated trait values are accessed by type, such as `Monoid.empty<MyList>`.

## Related Pages

- [[traits-and-implementations]]
- [[generics-and-kinds]]
- [[std-error]]
