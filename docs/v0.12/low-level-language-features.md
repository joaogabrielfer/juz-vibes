# Low-Level Language Features

This chapter collects explicitly low-level language escape hatches and VM-facing surface
rules.

## Inline Assembly

Standard library and performance-critical code can drop directly into SLUR stack-machine
instructions using `asm {}` blocks.

```rust
let fast_add :: int, int -> int;
let fast_add = a, b -> {
    asm { add }
}

let swap_top :: int, int -> (int, int);
let swap_top = a, b -> {
    asm { swap }
}

let mem_copy :: ptr, ptr, int -> unit;
let mem_copy = dst, src, len -> {
    asm { memcpy }
}
```

Inline assembly is reserved for standard-library authors and low-level systems code.
Normal application code should use standard-library abstractions instead.

## Named Prefix Notation

Low-level output and control names such as `print`, `echo`, `panic`, `todo`, and `cast`
are defined as named prefix notations in the standard prelude. They are not intrinsic
parser forms.

```rust
print "hello, world"
echo value
echo "hello, world"
panic "critical failure: state corrupted"
todo
cast<int>(float_value)
```

User-defined named notations may use the same syntax when declared with `@notation` and
the appropriate call-shape attributes.

See:

- [[expressions-and-operators]]
- [[std-prelude]]

## `print`, `println`, and `echo`

`print` is the lowest-level documented output surface for now. `println` and its variants
accept exactly one `string`.

```rust
print "Hello, world"
println("Hello, world!")
println("Hello " <> name <> "!")
println("Count: " <> Display::show(count))
println("Hello {name}, count: {count}")
```

`echo` remains generic over `Display`, calls `Display::show`, then prints the resulting
string.

```rust
echo value
echo user
```

`panic` prints the panic message and exits the current program with status code `1`.

```rust
panic "critical failure: state corrupted"
```

The declaration surfaces live in [[std-prelude]] and [[std-format]].
