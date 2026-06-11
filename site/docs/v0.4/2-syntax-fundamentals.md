## 2. Syntax Fundamentals

### Comments

```rs
// Single-line comment

/*
    Multi-line block comment.
    Can span any number of lines.
*/
```

### Identifiers

Standard alphanumeric identifiers with underscores. Conventions:

- `PascalCase` - types, traits, and modules
- `snake_case` - values, functions, and local bindings
- `SCREAMING_SNAKE_CASE` - compile-time constants
- `(operator_symbol)` - operator functions, wrapped in parentheses: `(+)`, `(<$>)`, `(|>?)`

### Semicolons

Semicolons appear in exactly two situations:

1. After non-final statements in a multi-statement `{}` block
2. After `return` and `be`

The final expression in a block has no semicolon and becomes the block value.
Top-level declarations do not need semicolons.

```rs
// No braces - no semicolons:
let add := (x, y) -> x + y

// Braces, single expression - no semicolon:
let add := (x, y) -> { x + y }

// Explicit return/be - semicolon after the keyword expression:
let add := (x, y) -> return x + y;
let add := (x, y) -> be x + y;

// Multi-statement block - semicolons between statements:
let process := x -> {
    let doubled := x * 2;
    let shifted := doubled + 16;
    shifted
}
```

### The Fat Arrow `=>`

`=>` is the single-line expression body selector. It is banned from top-level `let`
declarations and is only valid inside expression branches:

- `match` arms
- `if`/`else` one-liners
- `[]` pattern matching arms
- `when` guard bodies

```rs
// Invalid:
let f: (int) -> int => it * 2

// Valid branch body:
if x > 0       => echo "positive"
else if x == 0 => echo "zero"
else           => echo "negative"
```

### String Concatenation

`<>` appends strings and other monoidal values:

```rs
let greeting := "Hello " <> name <> "!"
```

String interpolation is handled by a built-in compile-time macro. See
[[20-metaprogramming-pipeline]].
