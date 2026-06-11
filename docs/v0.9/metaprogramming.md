# Metaprogramming

Element splits metaprogramming into distinct compile-time categories. Syntax macros,
compile-time elements, and transforms are not the same mechanism.

## Phases

Compilation proceeds through these phases:

1. Syntax macro expansion
2. AST construction
3. Comptime evaluation and transform expansion

If a compilation unit contains no syntax macro invocations, Phase 1 may be skipped.

## Syntax Macros

A syntax macro is a Phase-1 compile-time element declared with `@macro`. Syntax macros
operate on `TokenStream` and may transform arbitrary token input into new source tokens
before AST construction.

Syntax macro declarations must use a bang-suffixed name.

```rust
@macro
let json!: (TokenStream) -> TokenStream = stream -> {
    let validated := validate_json_tokens(stream);
    transform_to_element_ast(validated)
}

let config := json!({ "host": "localhost", "port": 8080 })
```

Syntax macros may only be invoked with bang syntax. Calling a syntax macro without `!` is
a compile error.

The default macro call form is ordinary parenthesized application. Brace-only calls
require explicit call-shape attributes.

```rust
let page := html!({
    <nav> ... </nav>
})
```

## Built-In String Interpolation

String interpolation is a built-in compiler-provided literal transform and requires no
import. It is macro-like semantically, but it is not a user-declared `@macro`.

Interpolation lowers at compile time, transforming `{expr}` inside string literals into
`<>` concatenation chains.

```rust
println("Hello {name}, you have {count} messages")
```

Equivalent lowering:

```rust
println("Hello " <> Display.show(name) <> ", you have " <> Display.show(count) <> " messages")
```

Format specifiers call formatting hooks:

```rust
println("Value: {n:.2f}")
println("Debug: {foo:?}")
```

Escaped braces use doubled delimiters.

```rust
println("{{literal brace}}")
```

Default lowering hooks:

- `{expr}` -> `Display.show(expr)`
- `{expr:?}` -> debug formatting hook
- `{expr:spec}` -> formatting hook with string specifier payload

## Call-Shape Attributes on Bang Calls

`@macro` and bang-callable `@comptime` elements may use notation-style call-shape
attributes to control invocation syntax.

```rust
@comptime
@position(prefix) @bind(right->path)
let embed!: (Path) -> Bytes = path -> { ... }

let shader := embed! "frag.spv"
```

Example with block capture:

```rust
@macro
@position(prefix) @bind(block->markup)
let html!: (TokenStream) -> TokenStream = markup -> { ... }

let page := html! {
    <nav> ... </nav>
}
```

## Comptime Elements

Any eligible element may be executed at compile time through `@run` if its inputs and
dependencies are compile-time available.

`@comptime` marks an element as compile-time-only.

```rust
@comptime
let build_lookup: (Arr<string>) -> Arr<(string, int)> = ops -> {
    ops |> map_indexed((op, i) -> (op, i))
}
```

A `@comptime` element:

- may execute only at compile time
- is a compile error in runtime-only execution
- may use compile-time-only APIs and meta-only types
- may be exported by plugins

Non-`@comptime` elements may still be evaluated at compile time through `@run` when
valid.

```rust
let read_into_string: (Path) -> string = p -> { ... }

let src := @run read_into_string("foo.jz")
```

## Bang-Callable Comptime Elements

A `@comptime` element may optionally use a bang-suffixed name. In that form, it exposes
compile-time bang-call syntax instead of ordinary `@run` call syntax.

```rust
@comptime
@position(prefix) @bind(right->path)
let embed!: (Path) -> Bytes = path -> { ... }

let shader := embed! "shaders/frag.spv"
```

Rules:

- a bang-suffixed `@comptime` element may only be invoked with `!`
- a non-bang `@comptime` element is invoked with `@run`
- bang call syntax does not make an element a syntax macro
- bang call syntax for `@comptime` elements executes in Phase 3, not Phase 1

## Custom Macro Syntax

The broader macro design includes domain-specific macros with custom syntax, not only
Element token transformations.

```rust
db |> fetch!({ SELECT * FROM users WHERE id = ${user_id} })
```

Required capabilities:

1. Declare token delimiters or trigger syntax
2. Parse arbitrary syntax up to a defined terminator
3. Report domain-specific errors
4. Integrate type safety, such as SQL return schemas

This remains open and should be handled in a dedicated metaprogramming design thread.

## Scoped Transforms

A transform is a Phase-3 compile-time rewrite attached to an explicit scope. Transforms
operate on typed compiler structures and return patches.

Supported transform attributes:

- `@transform(...)`
- `@module_transform(...)`
- `@package_transform(...)`

```rust
@transform(generate_inspector)
def PlayerComponent :: prod { ... }

@module_transform(verify_routes)
package app
module app.routes

@package_transform(verify_forms_and_routes)
package app
plugin first
```

A transform function must be `@comptime`.

```rust
@comptime
let generate_inspector: (Item) -> Patch<Item> = item -> { ... }
```

Transforms may not:

- create new syntax macros during Phase 3
- re-enter tokenization as a normal control-flow step
- mutate nodes outside their declared scope
- observe invalid intermediate states unless a meta API explicitly exposes them

Compilation must enforce deterministic transform ordering, fixed-point iteration limits,
cycle detection, and source-map preservation for generated diagnostics.

## `@auto_impl` Internals

`@auto_impl` is a built-in macro-based mechanism for automatically generating trait
implementations.

```rust
@auto_impl(Monoid(empty: 0, combine: (+)))
def Counter :: prod { count: int }

@auto_impl(Eq, Ord, Hash, Clone)
def Score :: prod { value: int }
```

The exact macro API exposed to user-defined derivation logic is still pending.

## Statement-Level Directives

```rust
@run fn_call()
@insert "other.jz"
@if CONDITION { ... }
@assert CONDITION

let DATA: Arr<u8>  = embed! "assets/icon.png"
let TABLE: Arr<int> = @run build_lookup_table()
```
