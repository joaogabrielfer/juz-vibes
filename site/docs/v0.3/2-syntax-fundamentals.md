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

- `PascalCase` — types, type traits, and modules
- `snake_case` — values, functions, and local bindings
- `SCREAMING_SNAKE_CASE` — compile-time constants
- `(operator_symbol)` — operator functions, wrapped in parentheses: `(+)`, `(<$>)`, `(|>?)`

### Semicolons

Semicolons are **mandatory** at the end of all statements inside `{}` block scopes. The
single exception is the **final expression** in a block, which omits its semicolon and
acts as the block's implicit return value:

```rs
let compute: (int, int) -> int = (base, scale) -> {
    let raw := base * scale;     // statement — semicolon required
    let adjusted := raw + 16;    // statement — semicolon required
    adjusted                     // final expression — no semicolon, implicitly returned
};
```

At module (top-level) scope, semicolons terminate declarations.

### The Fat Arrow =>

=> is the **single-line expression body selector**. It is strictly banned from top-level
`let` declarations and is only valid inside expression branches:

- `match` arms
- `if`/`else` one-liners
- `[]` pattern matching arms
- `when` guard bodies

```rs
// BANNED — compile error:
let f: (int) -> int => it * 2;

// ALLOWED — interior branch:
if x > 0   => echo "positive";
else if x == 0 => echo "zero";
else           => echo "negative";
```

---

