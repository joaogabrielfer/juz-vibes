## 12. Generics

```rs
// Simple generic:
let wrap: <T> (T) -> Option<T> = { .Some(it) };

// Single bound:
let show_all: <T: Display> (Arr<T>) -> string = items -> {
    items |> map(i -> i.show) |> join(", ")
};

// Multiple bounds with +:
let debug_log: <T: Display + Hash> (T) -> void = val -> {
    echo format("[{}] hash={}", val.show, val.hash)
};

// Generic type definition:
def Pair<A, B> = prod { first: A, second: B };

def Either<L, R> = sum {
    Left(L),
    Right(R),
};

// Generic with trait bounds on the type:
def @impl(Functor) Result<T, E> = extend {
    self: sum { Ok(T), Err(E) };

    map = <A, B> (f, r) -> {
        match r [
            .Ok(v)  => .Ok(f(v))
            .Err(e) => .Err(e)
        ]
    };
};

// Higher-kinded generics (container parameterizing an inner type):
let lift: <F: Functor, A, B> (def (A) -> B, F<A>) -> F<B> = (f, fa) -> {
    F.map(f, fa)
};
```

---

