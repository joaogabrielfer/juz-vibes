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

### Unparenthesized Intrinsics

Core VM intrinsics act as unary keywords at precedence level 10, permitting
parenthesis-free call syntax.

```rs
echo value
echo "hello, world"
panic "critical failure: state corrupted"
todo
cast<int>(float_value)
```

These are the only identifiers in Element callable without parentheses. All user-defined
functions require explicit call syntax. The precedence 10 placement means intrinsic calls
resolve before all binary operators.

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
