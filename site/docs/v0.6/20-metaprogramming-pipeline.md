## 20. Metaprogramming Pipeline

Element splits metaprogramming into distinct compile-time categories. Syntax macros,
compile-time elements, and transforms are not the same mechanism and must not be
conflated.

### Phases

Compilation proceeds through these phases:

1. Syntax Macro Expansion
2. AST Construction
3. Comptime Evaluation and Transform Expansion

Phase 1 operates on raw token streams before parsing. Phase 3 operates on typed compiler
structures after parsing.

If a compilation unit contains no syntax macro invocations, Phase 1 may be skipped.

### Syntax Macros

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

@macro
let sql!: (TokenStream) -> TokenStream = stream -> {
    parse_sql(stream) |> generate_prepared_statement
}

let config := json!({ "host": "localhost", "port": 8080 })
let query  := sql!({ SELECT * FROM users WHERE active = true })
```

Syntax macros may only be invoked with bang syntax. Calling a syntax macro without `!` is
a compile error.

The default macro call form is ordinary parenthesized application. A brace-only call such
as `html! { ... }` or `sql! { ... }` requires explicit call-shape attributes on the
macro declaration. The same rule applies to bang-callable `@comptime` elements.

Syntax macros:
- run only in Phase 1,
- receive tokens, not typed values,
- cannot inspect runtime or typed AST state,
- cannot trigger a return to Phase 1 from later phases.

### Built-In String Interpolation

String interpolation is a built-in compiler-provided literal transform and requires no
import. It is macro-like semantically, but it is not a normal user-declared `@macro`
because interpolation occurs inside string literals rather than at ordinary token-macro
call sites.

Interpolation lowers at compile time, transforming `{expr}` inside string literals into
`<>` concatenation chains.

```rs
println("Hello {name}, you have {count} messages")
```

The macro generates code equivalent to:

```rs
println("Hello " <> name.show <> ", you have " <> count.show <> " messages")
```

Format specifiers call formatting hooks:

```rs
println("Value: {n:.2f}")    // n.show_fmt(".2f")
println("Debug: {foo:?}")    // future debug trait
```

Escaped braces use doubled delimiters:

```rs
println("{{literal brace}}")
```

The default lowering hooks are:
- `{expr}` -> `expr.show`
- `{expr:?}` -> debug formatting hook
- `{expr:spec}` -> formatting hook with string specifier payload

String interpolation is specified as built-in syntax sugar rather than as a user-defined
macro API. Future customization, if added, should hook into formatting traits or
compiler-provided interpolation handlers rather than replacing the literal syntax itself.

### Call-Shape Attributes on Compile-Time Bang Calls

`@macro` and bang-callable `@comptime` elements may use notation-style call-shape
attributes to control invocation syntax. This does not replace `@notation`. It reuses the
same shape vocabulary for compile-time call forms.

Supported attributes include:
- `@position(...)`
- `@bind(...)`
- `@assoc(...)`
- `@precedence(...)`
- `@adjacent(...)`

Example:

```rs
@comptime
@position(prefix) @bind(right->path)
let embed!: (Path) -> Bytes = path -> { ... }

let shader := embed! "frag.spv"
```

Example with block capture:

```rs
@macro
@position(prefix) @bind(block->markup)
let html!: (TokenStream) -> TokenStream = markup -> { ... }

let page := html! {
    <nav> ... </nav>
}
```

Without call-shape attributes, the default invocation remains parenthesized:

```rs
let page := html!({
    <nav> ... </nav>
})
```

### Comptime Elements

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
- may execute only at compile time,
- is a compile error in runtime-only execution,
- may use compile-time-only APIs and meta-only types,
- may be exported by plugins.

Non-`@comptime` elements may still be evaluated at compile time through `@run` when
valid.

```rs
let read_into_string: (Path) -> string = p -> { ... }

let src := @run read_into_string("foo.jz")
```

### Bang-Callable Comptime Elements

A `@comptime` element may optionally use a bang-suffixed name. In that form, it exposes
compile-time bang-call syntax instead of ordinary `@run` call syntax.

```rs
@comptime
@position(prefix) @bind(right->path)
let embed!: (Path) -> Bytes = path -> { ... }

let shader := embed! "shaders/frag.spv"
```

Rules:
- a bang-suffixed `@comptime` element may only be invoked with `!`,
- a non-bang `@comptime` element is invoked with `@run`,
- bang call syntax does not make an element a syntax macro,
- bang call syntax for `@comptime` elements executes in Phase 3, not Phase 1.

These two declarations are distinct in kind:

```rs
@macro
let html!: (TokenStream) -> TokenStream = ...

@comptime
let embed!: (Path) -> Bytes = ...
```

`html!` is a syntax macro. `embed!` is a compile-time element with bang-call syntax.

### Custom Macro Syntax

The broader macro design includes domain-specific macros with custom syntax, not only
Element token transformations.

```rs
db |> fetch!({ SELECT * FROM users WHERE id = ${user_id} })
```

In this form, `${user_id}` captures a scoped variable safely into the domain-specific
syntax. The macro system must eventually support:

1. Declaring token delimiters or trigger syntax
2. Parsing arbitrary syntax up to a defined terminator
3. Reporting domain-specific errors
4. Integrating type safety, such as SQL return schemas

This area is open and should be handled in a dedicated metaprogramming design thread.

### Scoped Transforms

A transform is a Phase-3 compile-time rewrite attached to an explicit scope. Transforms
operate on typed compiler structures and return patches.

Supported transform attributes are:
- `@transform(...)`
- `@module_transform(...)`
- `@package_transform(...)`

Item transform:

```rs
@transform(generate_inspector)
def PlayerComponent :: prod { ... }
```

Module transform:

```rs
@module_transform(verify_routes)
package app
module app.routes
```

Package transform:

```rs
@package_transform(verify_forms_and_routes)
package app
plugin first
```

A transform function must be `@comptime`.

```rs
@comptime
let generate_inspector: (Item) -> Patch<Item> = item -> { ... }
```

Transforms are explicitly scoped:
- an item transform may rewrite the attached item and emit sibling items,
- a module transform may rewrite items in the current module and emit module-local items,
- a package transform may inspect the whole package and emit package-level items or
  modules.

Transforms may not:
- create new syntax macros during Phase 3,
- re-enter tokenization as a normal control-flow step,
- mutate nodes outside their declared scope,
- observe invalid intermediate states unless a meta API explicitly exposes them.

Compilation must enforce:
- deterministic transform ordering,
- fixed-point iteration limits,
- cycle detection,
- source-map preservation for generated diagnostics.

### `@auto_impl` Internals

`@auto_impl` is a built-in macro-based mechanism for automatically generating trait
implementations.

```rs
@auto_impl(Monoid(empty: 0, combine: (+)))
def Counter :: prod { count: int }

@auto_impl(Eq, Ord, Hash, Clone)
def Score :: prod { value: int }
```

The exact macro API exposed to user-defined derivation logic is still pending.

### Statement-Level Directives

```rs
@run fn_call()
@insert "other.jz"
@if CONDITION { ... }
@assert CONDITION

let DATA: Arr<u8>  = embed! "assets/icon.png"
let TABLE: Arr<int> = @run build_lookup_table()
```
