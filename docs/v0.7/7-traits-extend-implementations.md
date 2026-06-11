## 7. Traits, extends & Implementations

Element has one unified trait system. The syntax-level distinction between behavioral
traits and type traits has been abolished. Every trait is declared with:

```rs
def Name :: trait { ... }
```

The compiler determines at use-time whether a trait subelement is a method-like
subelement, because its type is a function, or a data subelement, because its type is a
value.

```rs
def Display :: trait {
    show: (Self) -> string,
}

def Monoid<T> :: trait {
    empty:   T,
    combine: (T, T) -> T,
}

def Monad<M> :: trait {
    bind: <A, B> (M<A>, def((A) -> M<B>)) -> M<B>,
}
```

### Implementation Form 1 - `extends`

`extends` combines a type definition with one or more trait implementations. The `self`
subelement defines the runtime data layout. Other subelements satisfy trait requirements
or define computed properties.

```rs
@impl(Display)
def Point :: extends {
    self: prod { x: int, y: int },
    show = "{self.x}, {self.y}",
}
```

A value of an `extends`-defined type takes exactly the memory of its `self` layout at
runtime. There is no boxing and no vtable.

Multiple traits can be implemented in the same block:

```rs
@impl(Monad, Applicative, Functor)
def Option<T> :: extends {
    self: sum {
        Some(T),
        None,
    },

    map = <A, B> (f, context) -> {
        match context [
            .Some(val) => .Some(f(val))
            .None      => .None
        ]
    },

    pure = val -> .Some(val),

    apply = <A, B> (wrapped_f, wrapped_val) -> {
        match wrapped_f [
            .Some(f) => f <$> wrapped_val
            .None    => .None
        ]
    },

    bind = <A, B> (context, f) -> {
        match context [
            .Some(val) => f(val)
            .None      => .None
        ]
    },
}
```

### Implementation Form 2 - `@auto_impl`

`@auto_impl` is a built-in macro for derived implementations.

```rs
@auto_impl(Monoid(empty: 0, combine: (+)))
def Score :: prod { value: int }

@auto_impl(Eq, Ord, Hash, Clone)
def Point :: prod { x: int, y: int }
```

The explicit-argument form is used when the implementation is not obvious from structure.
The argument-free form is structural derivation, comparable to Rust `derive`.

### Implementation Form 3 - Separate `let @impl`

`let @impl` declares implementation members separately from the type. A type signature is
optional when the trait already determines the subelement type.

```rs
def Point :: prod { x: int, y: int }

let @impl(Display) show :: (Point) -> string
let @impl(Display) show = self -> "{self.x}, {self.y}"
```

`let @impl` is the most flexible implementation form and is the desugaring target for
`extends`.

### Extending External Types

An existing type can receive trait implementations without changing its original layout.
When an `extends` block has no `self` subelement, `self` is implicitly the extended type.

```rs
@impl(Display)
def Option<T> :: extends {
    show = {
        match self [
            .Some(v) => "Some({v.show})"
            .None    => "None"
        ]
    },
}
```

Adding a `self` layout subelement when extending an existing type is a type error.

### Subelement Definition Rules

Inside `extends`, subelements that implement trait requirements omit type annotations when
the trait already declares their type.

```rs
def Display :: trait {
    show: (Self) -> string,
}

@impl(Display)
def User :: extends {
    self: prod { id: int, name: string },
    show = { "User({self.name})" },
}
```

Computed properties that are not known from a trait may carry explicit types:

```rs
@impl(Display)
def Time :: extends {
    self: int,
    ms: int = { self * 1_000 },
    show = { "{self}s" },
}
```

### Pattern Matching on Implementations

`[]` blocks are valid on the right side of `let @impl`. The compiler applies
exhaustiveness checking against the sum type's variants.

```rs
def Area :: trait { area: (Self) -> float }

let @impl(Area) area = [
    (self: Shape.Circle)   => PI * self.r * self.r
    (self: Shape.Rect)     => self.w * self.h
    (self: Shape.Triangle) => 0.5 * self.base * self.height
]
```

Adding a new `Shape` variant becomes a compile error until the implementation handles it.

### Family Requirements with `@requires(...)`

Type declarations may require traits on later nested members:

```rs
@requires(Display)
def Error :: expandable sum
```

For declarations in the same package, required-trait validation happens at package scope.
Members may be declared in one file and implemented in another file of the same package.
The package fails validation if required traits are still missing at the end of package
checking.

### Trait Coherence and Overrides

Implicit priority is:

1. Local definitions
2. Plugin definitions
3. Standard library definitions

`@override` is required only for conflicts at the same level.

```rs
@override(priority: 2)
@impl(Display)
def SomeExternalType :: extends {
    show = { ... },
}

@override(priority: 2)
let @impl(Display) show = self: SomeExternalType -> { ... }
```

If a user shadows a standard trait implementation without `@override`, the compiler emits
a warning:

```txt
Display for Option<T> has been overridden by your_module - use @override(priority) to make this explicit.
```

Rules:

| Situation | Compiler action |
|---|---|
| One implementation, no conflict | Accepted |
| Two same-level implementations, no override | Compile error |
| Two same-level overrides, different priorities | Higher priority wins |
| Two same-level overrides, same priority | Compile error |
