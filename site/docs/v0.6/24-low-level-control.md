## 24. Low-Level Control

### Inline Assembly

Standard library and performance-critical code can drop directly into SLUR stack-machine
instructions using `asm {}` blocks.

```rs
let fast_add: (int, int) -> int = (a, b) -> {
    asm { add }
}

let swap_top: (int, int) -> (int, int) = (a, b) -> {
    asm { swap }
}

let mem_copy: (ptr, ptr, int) -> void = (dst, src, len) -> {
    asm { memcpy }
}
```

Inline assembly is reserved for standard library authors and low-level systems code.
Normal application code should use standard library abstractions instead.

### Named Prefix Notation

Core VM operations such as `echo`, `panic`, `todo`, and `cast` are defined as named prefix
notations. They can be used without parentheses when declared with
`@position(prefix) @bind(right->...)`.

```rs
echo value
echo "hello, world"
panic "critical failure: state corrupted"
todo
cast<int>(float_value)
```

Other user-defined named notations may also be callable without parentheses when declared
with the same notation rules. This behavior is controlled by `@notation`, not by intrinsic
special-casing. See [[15-notation-system]].

### `println` and `echo`

`println` and its variants accept exactly one `string`.

```rs
println("Hello, world!")
println("Hello " <> name <> "!")
println("Count: " <> count.show)
println("Hello {name}, count: {count}")
```

`echo` remains generic over `Display` and calls `.show` internally.

```rs
echo value
echo user
```
