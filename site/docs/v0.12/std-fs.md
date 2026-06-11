# std::fs

This page is a placeholder for the standard filesystem surface.

Filesystem operations are expected to perform `IO`. Concrete path, file, directory, and
metadata APIs remain open in this revision.

## Future Path Ergonomics

The future `Path` surface should consider path-join notation through `/`.

```rust
let config_path: Path = fs::home_dir_path / ".config"
```

This notation should be defined only for `Path`-like values. It must not make ordinary
string division meaningful.
