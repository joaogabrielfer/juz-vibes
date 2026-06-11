# Lexical Syntax

This chapter owns token-level and surface syntax rules that apply before semantic
resolution.

## Comments

```rust
// Single-line comment

/*
    Multi-line block comment.
    Can span any number of lines.
*/
```

## Identifiers

Standard identifiers are alphanumeric with underscores.

Conventions:

- `PascalCase` for types, traits, and modules
- `snake_case` for values, functions, and local bindings
- `SCREAMING_SNAKE_CASE` for compile-time constants
- `(notation_name_or_symbol)` for notation values, such as `(+)`, `(echo)`, and `(<$>)`

Notation declarations do not change tokenization. `echofoo` is always one identifier
token and is never split into `echo foo`.

## Semicolons

Semicolons appear in exactly two situations:

1. After non-final statements in a multi-statement `{}` block.
2. After `return` and `be`.

The final expression in a block has no semicolon and becomes the block value.
Top-level declarations do not need semicolons.

```rust
let add := (x, y) -> x + y

let process := x -> {
    let doubled := x * 2;
    let shifted := doubled + 16;
    shifted
}

let early := x -> {
    if x == 0 {
        return 0;
    };

    x * 10
}
```

## Branch Arrow

`=>` is the single-line expression body selector for branch-like forms. It is banned from
top-level `let` declarations.

Valid contexts:

- `match` arms
- `if` and `else` one-liners
- `[]` pattern-function arms
- `when` guard bodies

Invalid:

```rust
let f: (int) -> int => it * 2
```

Valid:

```rust
if x > 0       => echo "positive"
else if x == 0 => echo "zero"
else           => echo "negative"
```

## Strings

String concatenation and interpolation are expression-level features documented in
[[expressions-and-operators]] and [[metaprogramming]].
