# Traits and Implementations

Element has one unified trait system. Every trait is declared with:

```rust
def Name :: trait { ... }
```

The compiler determines at use-time whether a trait subelement is method-like because its
type is a function, or data-like because its type is a value.

```rust
def Display :: trait {
    show: (Self) -> string,
}

def Monoid<T> :: trait {
    empty:   T,
    combine: (T, T) -> T,
}
```

Canonical standard traits are documented in [[std-core-traits]].

## `extends`

`extends` defines trait implementations. In the preferred 2-in-1 form, the `self`
subelement also defines the runtime representation of the type. Trait members and
structural subelements live in distinct namespaces.

```rust
def Point :: extends {
    self: prod {
        x: int,
        y: int,
        show: string = "{self.x}, {self.y}",
    },

    @impl(Display) {
        show = self.show
    }
}
```

A value of an `extends`-defined type takes exactly the memory of its `self` layout at
runtime. There is no boxing and no vtable. Trait members declared inside `@impl(...)`
blocks do not become dot-accessible members of the type.

Multiple traits can be implemented in the same block.

```rust
def Option<T> :: extends {
    self: sum {
        Some(T),
        None,
    },

    @impl(Functor) {
        map = <A, B> (f, context) -> {
            match context [
                .Some(val) => .Some(f(val))
                .None      => .None
            ]
        }
    }

    @impl(Applicative) {
        pure = val -> .Some(val)
        apply = <A, B> (wrapped_fn, wrapped_val) -> {
            match wrapped_fn, wrapped_val [
                (.Some(f), .Some(v)) => .Some(f(v))
                _                    => .None
            ]
        }
    }

    @impl(Monad) {
        bind = <A, B> (context, f) -> {
            match context [
                .Some(val) => f(val)
                .None      => .None
            ]
        }
    }
}
```

An existing product declaration may also be extended separately:

```rust
def Point :: prod {
    x: int,
    y: int,
    show: string = "{self.x}, {self.y}",
}

def Point :: extends {
    @impl(Display) {
        show = self.show
    }
}
```

## Trait Namespace

Dot access resolves only structural subelements declared by the type.

```rust
let p := Point(10, 20)

println(p.show)
println(Display.show(p))
```

Trait lookup never happens through dot access. If a type has a structural subelement
named `show` and also implements `Display.show`, those remain distinct names.

Instance-style trait members are accessed through the trait namespace:

```rust
Display.show(p)
Monoid.combine(a, b)
```

Associated trait values are accessed by type:

```rust
Monoid.empty<Point>
```

## `@auto_impl`

`@auto_impl` is a built-in macro-based mechanism for derived implementations.

```rust
@auto_impl(Monoid(empty: 0, combine: (+)))
def Score :: prod { value: int }

@auto_impl(Eq, Ord, Hash, Clone)
def Point :: prod { x: int, y: int }
```

The explicit-argument form is used when the implementation is not obvious from structure.
The argument-free form is structural derivation.

## Separate `let @impl`

`let @impl` declares implementation members separately from the type. A type signature is
optional when the trait already determines the subelement type.

```rust
def Point :: prod {
    x: int,
    y: int,
    show: string = "{self.x}, {self.y}",
}

let @impl(Display) show :: (Point) -> string
let @impl(Display) show = self -> self.show
```

`let @impl` is the most flexible implementation form and is the desugaring target for
`extends`.

## Extending External Types

An existing type can receive trait implementations without changing its original layout.
When an `extends` block has no `self` subelement, `self` is implicitly the extended type.

```rust
def Option<T> :: extends {
    @impl(Display) {
        show = {
            match self [
                .Some(v) => "Some({Display.show(v)})"
                .None    => "None"
            ]
        }
    }
}
```

Adding a `self` layout subelement when extending an existing type is a type error.

## Subelement Definition Rules

Structural subelements belong in the type body, not inside `@impl(...)` blocks.

```rust
def User :: prod {
    id: int,
    name: string,
    show: string = "User({self.name})",
}

def User :: extends {
    @impl(Display) {
        show = self.show
    }
}
```

Self-derived defaults are evaluated during construction and stored in the instance. They
may use explicit type annotations or `:=` when the type is obvious.

```rust
def Time :: prod {
    seconds: int,
    ms: int = self.seconds * 1_000,
    show: string = "{self.seconds}s",
}
```

## Pattern Matching on Implementations

`[]` blocks are valid on the right side of `let @impl`. The compiler applies
exhaustiveness checking against the sum type's variants.

```rust
def Area :: trait { area: (Self) -> float }

let @impl(Area) area = [
    (self: Shape.Circle)   => PI * self.r * self.r
    (self: Shape.Rect)     => self.w * self.h
    (self: Shape.Triangle) => 0.5 * self.base * self.height
]
```

Adding a new `Shape` variant becomes a compile error until the implementation handles it.

## Family Requirements

Type declarations may require traits on later nested members.

```rust
@requires(Display)
def Error :: expandable sum
```

For declarations in the same package, required-trait validation happens at package scope.
The package fails validation if required traits are still missing at the end of package
checking.

## Coherence and Overrides

Implicit priority is:

1. Local definitions
2. Plugin definitions
3. Standard library definitions

`@override` is required only for conflicts at the same level.

```rust
@override(priority: 2)
def SomeExternalType :: extends {
    @impl(Display) {
        show = { ... }
    }
}
```

Rules:

| Situation | Compiler action |
|---|---|
| One implementation, no conflict | Accepted |
| Two same-level implementations, no override | Compile error |
| Two same-level overrides, different priorities | Higher priority wins |
| Two same-level overrides, same priority | Compile error |
