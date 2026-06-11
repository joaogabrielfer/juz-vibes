## 7. Traits, extend & Implementations

Element has two distinct categories of traits that serve fundamentally different purposes.

### Behavioral Traits

Define method signatures a type must implement. These are interface contracts — comparable
to Rust traits. Declared with `def Name = trait {}`. The special type `Self` refers to
the implementing type:

```rs
def Display = trait {
    show: (Self) -> string,
};

def Eq<T> = trait {
    equals: (T, T) -> bool,
};

def Ord<T> = trait {
    compare: (T, T) -> int,
};

def Hash = trait {
    hash: (Self) -> u64,
};
```

### Type Traits

Define structural/data fields a type must carry. These fields exist **only in the
compiler's static type registry** — they are completely erased from the output binary.
Type traits enable higher-kinded abstractions (Functor, Monad, Monoid, etc.):

```rs
def Monoid<T> = trait {
    empty:   T,
    combine: (T, T) -> T,
};

def Semigroup<T> = trait {
    combine: (T, T) -> T,
};

def Functor<F> = trait {
    map: <A, B> (def (A) -> B, F<A>) -> F<B>,
};

def Applicative<F> = trait {
    pure:  <A> (A) -> F<A>,
    apply: <A, B> (F<def (A) -> B>, F<A>) -> F<B>,
};

def Monad<M> = trait {
    bind: <A, B> (M<A>, def (A) -> M<B>) -> M<B>,
};

def Collection<C, Item> = trait {
    next: (C) -> Result<Item, string>,
};
```

### `extend` — Type Definition with Trait Implementations

`extend` declares a new type and simultaneously satisfies one or more traits. The `self`
block defines the actual runtime data layout. All other fields satisfy trait requirements
and are erased at compile time.

A variable of an `extend`-defined type takes up exactly the memory of its `self` block
at runtime — no boxing, no vtables, no overhead:

```rs
def @impl(Monad, Applicative, Functor) Option<T> = extend {
    self: sum {
        Some(T),
        None,
    };

    // Functor
    map = <A, B> (f, context) -> {
        match context [
            .Some(val) => .Some(f(val))
            .None      => .None
        ]
    };

    // Applicative
    pure  = val -> .Some(val);
    apply = <A, B> (wrapped_f, wrapped_val) -> {
        match wrapped_f [
            .Some(f) => f <$> wrapped_val
            .None    => .None
        ]
    };

    // Monad
    bind = <A, B> (context, f) -> {
        match context [
            .Some(val) => f(val)
            .None      => .None
        ]
    };
};
```

#### Field Definition Syntax Inside `extend`

Fields follow the same rules as `let` bindings but without the `let` keyword. Two valid
forms:

```rs
def @impl(Display) Point = extend {
    self: prod { x: int, y: int };

    // Short form — type inferred from trait definition, self is implicit:
    show = { format("({}, {})", self.x, self.y) };

    // Long form — explicit types and argument name:
    show: (s: Self) -> string = { format("({}, {})", s.x, s.y) };
};
```

In the short form, `self` is implicitly bound to the current instance. In the long form,
you explicitly name the argument (here `s`), and `self` is no longer implicit — use `s`
in the body instead.

### Implementing Behavioral Traits on Existing Types

A type defined with `prod`, `sum`, or other keywords can receive trait implementations
at any point, including types from other modules. The compiler forbids modifying the
original `self` layout:

```rs
// Defined in std — the original type:
def Option<T> = sum { Some(T), None };

// In your module — extend without self block:
// self is implicitly Option<T> since the type already exists
def @impl(Display) Option<T> = extend {
    show = {
        match self [
            .Some(v) => format("Some({})", v.show)
            .None    => "None"
        ]
    };
    // Adding a self: block here is a compile error —
    // you cannot change an existing type's layout
};

// Multiple traits at once:
def @impl(Display, Hash) Point = extend {
    show = { format("({}, {})", self.x, self.y) };
    hash = { hash_combine(self.x, self.y) };
};
```

### `@auto_impl` — Derived Implementations

A built-in macro that generates `extend` blocks for type traits with straightforward field
mappings. Equivalent to Rust's `#[derive(...)]` but user-extensible:

```rs
@auto_impl(Monoid(empty: 0, combine: (+)))
def Score = prod {
    value: int,
};

// Compiler generates:
// def @impl(Monoid) Score = extend {
//     self: prod { value: int };
//     empty   = 0;
//     combine = (+);
// };

// Multiple:
@auto_impl(Eq(equals: (==)), Ord(compare: int_compare))
def Priority: enum<int> { Low = 1, Medium, High }
```

### Trait Coherence & `override`

Each trait can be implemented once per type per module boundary. Conflicts require explicit
resolution using `override`:

```rs
// Two modules implement Display for ExternalType:
override(priority: 2) impl ExternalType for Display = module_a_impl;

// Rules:
// — One impl, no conflict                     → no annotation needed
// — Two impls, no override                    → compile error
// — Two overrides, different priorities       → higher priority wins
// — Two overrides, same priority              → compile error
```

---

