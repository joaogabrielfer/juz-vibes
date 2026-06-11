# Markdown Conventions

This wiki is edited in Obsidian.

## Code Fence Tags

Use `rust` for Element examples because Obsidian highlights Element-like syntax best with
the Rust highlighter.

```rust
let add := (x, y) -> x + y
```

Use `txt` only when the block is not Element code:

- grammar shapes
- CLI command shapes
- compiler diagnostics
- plain output
- directory layouts

Examples:

```txt
let [mut] [attrs] name: <generics> (args) -> return_type = body
```

```txt
pyx <file> [ini_name]
pyx init <name> <template>
```

```txt
error: expected kind 'Type -> Type'
found: 'Type'
```

## Dataview Compatibility

When a short inline example contains `=` near backticks, prefer single quotes instead of
inline code. Obsidian Dataview can interpret backtick-equals sequences unexpectedly.

## Tables

When documenting notation that contains `|`, escape it inside Markdown tables as `\|`.
