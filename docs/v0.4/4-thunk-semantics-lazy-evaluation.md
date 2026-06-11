## 4. Thunk Semantics & Lazy Evaluation

Every zero-argument binding is a **managed thunk**. Two evaluation modes govern how and
when the body is executed.

### Default - First-Access Caching

A zero-argument element evaluates its body exactly once, on the first bare access in its
scope. All subsequent bare accesses return the cached value. Appending `()` forces a fresh
re-evaluation, bypassing the cache:

```rs
let @pub timestamp: int = get_unix_time()

let t1 := timestamp    // body evaluated here, result cached
let t2 := timestamp    // cache hit, same value as t1
let t3 := timestamp()  // forces fresh evaluation
```

`foo` accesses the cached value. `foo()` calls the element again.

### `@lazy` - Always Re-Evaluate

The `@lazy` attribute removes the internal cache entirely. Every bare access triggers a
fresh execution of the body. For a `@lazy` element, `foo` and `foo()` are equivalent:

```rs
let @pub @lazy live_clock: int = get_unix_time()

let c1 := live_clock
let c2 := live_clock
let c3 := live_clock()
```

### Computed Properties

The thunk model makes computed properties ergonomic without method-call syntax. A computed
property is a zero-argument element associated with a type implementation. It is accessed
with dot notation and cached according to the same rules as any other thunk.

```rs
@impl(Display)
def Time :: extends {
    self: int,

    ms:   int = { self * 1_000 },
    us:   int = { self * 1_000_000 },
    ns:   int = { self * 1_000_000_000 },
    days: int = { self / 86_400 },

    show = { format("{}s", self) },
}

let t := now()

echo t.ms
echo t.days
echo t.show
```

There are no method calls. Dot access either reads a stored field or evaluates a
zero-argument computed property.
