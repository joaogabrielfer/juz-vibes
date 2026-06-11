## 4. Thunk Semantics & Lazy Evaluation

Every zero-argument binding is a **managed thunk**. Two evaluation modes govern how and
when the body is executed.

### Default — First-Access Caching

A zero-argument element evaluates its body exactly once, on the first bare access in its
scope. All subsequent bare accesses return the cached value. Appending `()` forces a fresh
re-evaluation, bypassing the cache:

```rs
let @pub timestamp: int = get_unix_time();

let t1 := timestamp;    // body evaluated here — result cached
let t2 := timestamp;    // cache hit — same value as t1
let t3 := timestamp();  // forces fresh evaluation — new unix timestamp
```

`foo` accesses the value. `foo()` calls the function again.

### `@lazy` — Always Re-Evaluate

The `@lazy` attribute removes the internal cache entirely. Every bare access triggers a
fresh execution of the body. For a `@lazy` element, `foo` and `foo()` are equivalent:

```rs
let @pub @lazy live_clock: int = get_unix_time();

let c1 := live_clock;    // evaluated dynamically
let c2 := live_clock;    // evaluated again — different result
let c3 := live_clock();  // same as bare access for @lazy
```

### Practical Application — Computed Properties

The thunk model makes computed properties ergonomic without method-call syntax. The `Time`
type demonstrates this clearly:

```rs
def @impl(Display) Time = extend {
    self: int;   // the one real value: unix timestamp in seconds

    ms:   int = { self * 1_000 };
    us:   int = { self * 1_000_000 };
    ns:   int = { self * 1_000_000_000 };
    days: int = { self / 86_400 };

    show: string = { format("{}s", self) };
};

let t := now();

echo t.ms;    // computed on first access, cached — reads like a field
echo t.days;  // same
echo t.ns;    // same
echo t.show;  // "1718000000s"
```

Each computed property is evaluated once on first access, then cached for the lifetime of
that binding. There are no parentheses, no method call semantics, no cognitive overhead.

---

