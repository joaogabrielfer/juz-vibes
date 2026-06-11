## 12. Generics

```rs
let wrap: <T> (T) -> Option<T> = { .Some(it) }

let show_all: <T: Display> (Arr<T>) -> string = items -> {
    items |> map(i -> i.show) |> join(", ")
}

let debug_log: <T: Display + Hash> (T) -> void = val -> {
    echo "[{val.show}] hash={val.hash}"
}
```

### Generic Type Definitions

```rs
def Pair<A, B> :: prod { first: A, second: B }

def Either<L, R> :: sum {
    Left(L),
    Right(R),
}
```

### Generic Trait Implementations

```rs
@impl(Functor)
def Result<T, E> :: extends {
    self: sum { Ok(T), Err(E) },

    map = <A, B> (f, r) -> {
        match r [
            .Ok(v)  => .Ok(f(v))
            .Err(e) => .Err(e)
        ]
    },
}
```

### Higher-Kinded Generics

Higher-kinded generics allow a type parameter to represent a container that itself accepts
an inner type.

The full `Kind` model, explicit kind syntax (`F :: Type -> Type`), sugar syntax (`F<_>`),
and type-constructor partial application rules are defined in
[[25-kinds-higher-kinded-types]].

```rs
let lift:
    <F: Functor, A, B> (def((A) -> B), F<A>) -> F<B> =
    (f, fa) -> {
        F.map(f, fa)
    }
```
