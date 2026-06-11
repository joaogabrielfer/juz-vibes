# std.pvm

This page documents the VM-facing runtime module currently exposed to build scripts and
hosted tooling.

## Draft Surface

```rust
module std.pvm

let run: string -> Fallible<void>
```

## Notes

- `pvm.run("build/app.pbc")` is the only stable documented surface today.
- The broader embedding, inspection, and process-control API remains intentionally
  unspecified.

## Related Pages

- [[runtime-pvm]]
- [[open-questions]]
