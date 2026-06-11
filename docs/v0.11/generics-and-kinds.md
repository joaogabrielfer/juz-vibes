# Generics and Kinds

This chapter owns generic parameter syntax, trait bounds, kind checking, and type
constructor classification.

## Generic Functions

```rust
let wrap: <T> T -> Option<T> = { .Some(it) }

let show_all: <T: Display> (items: Arr<T>) -> string = {
    items |> map(i -> Display::show(i)) |> join(", ")
}

let debug_log: <T: Display + Hash> T -> void = val -> {
    println("[{val}] hash={Hash::hash(val)}")
}
```

## Generic Type Definitions

```rust
def Pair<A, B> :: prod { first: A, second: B }

def Either<L, R> :: sum {
    Left(L),
    Right(R),
}
```

## Generic Trait Implementations

```rust
def Fallible<T> :: extends {
    self: sum { Ok(T), Err(Error) },

    @impl(Functor) {
        map = <A, B> (f, r) -> {
            match r [
                .Ok(v)  => .Ok(f(v))
                .Err(e) => .Err(e)
            ]
        }
    },
}
```

## Type and Kind

`Type` and `Kind` are distinct language concepts.

- `Type` classifies values and concrete type expressions.
- `Kind` classifies type expressions and type constructors.

```rust
int               : Type
Option            : Type -> Type
Fallible          : Type -> Type
Either            : Type -> Type -> Type
Either<Error, _>  : Type -> Type
Option<int>       : Type
```

`Kind` values are compile-time only. A `Kind` value cannot exist at runtime.

## Kind Grammar

This is grammar notation, not Element code:

```txt
Kind ::= Type | Kind -> Kind | (Kind)
```

## Explicit and Inferred Kind Syntax

Explicit form, preferred for core and standard-library traits:

```rust
def Functor<F :: Type -> Type> :: trait {
    map: <A, B> (A -> B), F<A> -> F<B>
}
```

Sugar form:

```rust
def Functor<F<_>> :: trait {
    map: <A, B> (A -> B), F<A> -> F<B>
}
```

Both forms normalize to the same internal kind representation.

## Type Constructor Partial Application

`_` is allowed in type argument lists for partial application.

```rust
Fallible             // Type -> Type
Either<Error, _>     // Type -> Type
```

A type error is produced when partial application arity is invalid.

## Compile-Time Kind Introspection

```rust
@comptime
let k_opt = kind_of(Option)

let k_fall = @run kind_of(Fallible)
let k_either = @run kind_of(Either)
```

If a `Kind` expression escapes into a runtime path, the compiler must reject it.

## Type-Level Composition

Deeply nested type syntax may be expressed with type-level piping in type-expression
contexts.

```rust
(
  User
  |> Payload
  |> Fallible
  |> Async
)
```

This form is valid anywhere an anonymous type expression is valid:

```rust
let process: (User |> Payload |> Fallible |> Async) = ...
```

Named alias form:

```rust
def UserFetch :: {
  User
  |> Payload
  |> Fallible
  |> Async
}
```

These forms are equivalent to:

```rust
def UserFetch :: Async<Fallible<Payload<User>>>
```

Type-level `|>` must be parsed only inside type-expression contexts.

## Trait Constraints

Generic trait constraints remain in the existing style:

```rust
<T: Display + Hash>
```

No `where` syntax is introduced in this revision.

## Higher-Order Traits

Higher-order traits abstract over constructors that themselves take constructors.

```rust
def FunctorK<T :: (Type -> Type) -> Type> :: trait {
    mapK:
      <F :: Type -> Type, G :: Type -> Type>
      (<A> F<A> -> G<A>), T<F> -> T<G>
}
```

Example shape:

```rust
def UserRepo<F :: Type -> Type> :: trait {
    find_user: UserId -> F<Fallible<User>>
    save_user: User -> F<Fallible<void>>
}
```

## Diagnostics

Compiler diagnostic examples use `txt` because they are compiler output, not Element
code.

```txt
error: expected kind 'Type -> Type' for parameter 'F'
found: 'Type'
note: 'int' is a concrete type, not a type constructor
```

```txt
error: value of type 'Kind' cannot exist at runtime
help: move this expression to a compile-time evaluable context (@comptime or @run)
```
