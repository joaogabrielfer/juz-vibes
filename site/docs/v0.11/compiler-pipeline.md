# Compiler Pipeline

This page documents the compilation flow from parse-adjacent phases through package-level
transforms.

## Current Phase Ladder

1. Syntax macro expansion
2. AST construction
3. Comptime evaluation and transform expansion

## Scope

This page is the toolchain-oriented companion to [[metaprogramming]].

- [[metaprogramming]] owns the language forms
- this page owns the pipeline view a toolchain maintainer needs

## Current Responsibilities

- syntax macro scheduling
- transform scope boundaries
- deterministic ordering
- fixed-point iteration and cycle diagnostics
- source-map preservation for generated diagnostics

## Related Pages

- [[metaprogramming]]
- [[std-compiler]]
- [[open-questions]]
