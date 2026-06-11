## 25. Kinds & Higher-Kinded Types

`Kind` and `Type` are distinct language concepts.

- `Type` classifies values and concrete type expressions.
- `Kind` classifies type expressions and type constructors.

Examples:

```rust
int               : Type
Option            : Type -> Type
Result            : Type -> Type -> Type
Result<_, Error>  : Type -> Type
Option<int>       : Type
```

`Kind` values are compile-time only. A `Kind` value cannot exist at runtime.

### Kind Grammar

```txt
Kind ::= Type | Kind -> Kind | (Kind)
```

### Explicit and Inferred Kind Syntax

Element supports both explicit and sugar forms.

Explicit form (preferred for core and stdlib traits):

```rust
def Functor<F :: Type -> Type> :: trait {
    map: <A, B> (def((A) -> B), F<A>) -> F<B>
}
```

Sugar form:

```rust
def Functor<F<_>> :: trait {
    map: <A, B> (def((A) -> B), F<A>) -> F<B>
}
```

Both forms must normalize to the same internal kind representation.

### Type Constructor Partial Application

`_` is allowed in type argument lists for partial application.

```rust
Result<_, HttpErr>   // Type -> Type
Either<Err, _>       // Type -> Type
```

A type error is produced when partial application arity is invalid.

### Compile-Time Kind Introspection

`Kind` introspection is allowed in any compile-time evaluable context, including
`@comptime` declarations and `@run` calls evaluated at compile time.

```rust
@comptime
let k_opt = kind_of(Option) // Type -> Type

let k_res = @run kind_of(Result) // Type -> Type -> Type
```

If a `Kind` expression escapes into a runtime path, the compiler must reject it.

### `Type` and `Kind` Separation

`Type` reflection remains responsible for concrete layout and shape metadata:
`size`, `align`, `subelements`, `variants`, and related information.

`Kind` reflection is responsible for constructor-shape classification.

The compiler must not treat constructors as runtime layout-bearing `Type` values.

Meta-only compiler types such as `Item`, `Module`, `Package`, `Code`, and `TokenStream`
belong to the compile-time meta layer rather than the ordinary `Type`/`Kind` model used
for runtime-denotable programs.

### Type-Level Composition for Nested Types

Deeply nested type syntax may be expressed with type-level piping in type-expression
contexts.

`def(...)` is the anonymous type-expression form for type-level pipelines, parallel to
anonymous function types such as `def((int) -> string)`.

```rust
def (
  User
  |> Payload
  |> Result<_, HttpErr>
  |> Async
)
```

This form is valid anywhere an anonymous type expression is valid:

```rust
let process: def(User |> Payload |> Result<_, HttpErr> |> Async) = ...
```

Named alias form:

```rust
def UserFetch :: {
  User
  |> Payload
  |> Result<_, HttpErr>
  |> Async
}
```

These forms are equivalent to:

```rust
def UserFetch :: Async<Result<Payload<User>, HttpErr>>
```

Type-level `|>` must be parsed only inside type-expression contexts so it remains
unambiguous with value-level pipelines.

### Trait Constraints

Generic trait constraints remain in the existing style:

```rust
<T: Display + Hash>
```

No `where` syntax is introduced in this revision.

### Functor, Applicative, Monad with Kinds

```rust
def Functor<F :: Type -> Type> :: trait {
    map: <A, B> (def((A) -> B), F<A>) -> F<B>
}

def Applicative<F :: Type -> Type> :: trait {
    pure: <A> (A) -> F<A>
    apply: <A, B> (F<def((A) -> B)>, F<A>) -> F<B>
}

def Monad<M :: Type -> Type> :: trait {
    pure: <A> (A) -> M<A>
    bind: <A, B> (M<A>, def((A) -> M<B>)) -> M<B>
}
```

### Law Expectations

The compiler checks signatures, not algebraic laws. The language spec still requires
implementations to satisfy their trait laws.

Functor laws:

1. `map(id, fa) == fa`
2. `map(g >> f, fa) == map(g, map(f, fa))`

Applicative and Monad law sets are normative for standard abstractions. Compiler-side
proof is not required in this revision.

### `<$>` Operator Semantics

`<$>` is infix functor mapping.

```rust
f <$> fa
```

desugars to:

```rust
Functor.map(f, fa)
```

It must preserve context shape and only transform contained values.

Examples:

```rust
(+1) <$> .Some(2)      // .Some(3)
show <$> .Ok(42)       // .Ok("42")
show <$> .Err("oops")  // .Err("oops")
```

### Complex Example: Nested Context Mapping

```rust
def HttpErr :: sum { Timeout, Decode, Unauthorized }

def Payload<A> :: prod {
    trace_id: string,
    body: A,
}

@impl(Functor)
def Async<T> :: extends {
    self: sum {
        Ready(T),
        Pending,
        Failed(string),
    },

    map = <A, B> (f, x) -> {
        match x [
            .Ready(v)  => .Ready(f(v))
            .Pending   => .Pending
            .Failed(e) => .Failed(e)
        ]
    },
}

let transform_user:
    (Async<Result<Payload<User>, HttpErr>>) ->
    Async<Result<Payload<UserView>, HttpErr>> =
    stream -> {
        stream <$> (r -> r <$> (p -> Payload(trace_id: p.trace_id, body: to_view(p.body))))
    }
```

### Higher-Order Traits

Higher-order traits abstract over constructors that themselves take constructors.

```rust
def FunctorK<T :: (Type -> Type) -> Type> :: trait {
    mapK:
      <F :: Type -> Type, G :: Type -> Type>
      (def(<A> (F<A>) -> G<A>), T<F>) -> T<G>
}
```

`FunctorK` is useful when transforming an entire effect-parameterized module from one
effect constructor to another.

Example shape:

```rust
def UserRepo<F :: Type -> Type> :: trait {
    find_user: (UserId) -> F<Result<User, RepoErr>>
    save_user: (User) -> F<Result<(), RepoErr>>
}
```

With `FunctorK`, implementations can rewrite `UserRepo<F>` into `UserRepo<G>` through a
natural transformation instead of rewriting every operation manually.

### `Kind` Display

`Kind` supports `Display` for compile-time diagnostics and metaprogramming output.
Canonical renderings include:

- `Type`
- `Type -> Type`
- `(Type -> Type) -> Type`

Displaying a `Kind` is compile-time only and follows the same runtime restriction as all
`Kind` values.

### Diagnostics

Representative compiler diagnostics:

```txt
error: expected kind 'Type -> Type' for parameter 'F'
found: 'Type'
note: 'int' is a concrete type, not a type constructor
```

```txt
error: value of type 'Kind' cannot exist at runtime
help: move this expression to a compile-time evaluable context (@comptime or @run)
```
