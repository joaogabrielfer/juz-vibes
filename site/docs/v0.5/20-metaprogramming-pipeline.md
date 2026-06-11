## 20. Metaprogramming Pipeline

Element splits compilation into isolated processing phases. Each phase has strict
boundaries to prevent information leaking backward in the pipeline.

### Phase 1: Token Macros (`@macro`)

Token macros run on raw, unparsed token streams before the AST is constructed. They are
declared with `@macro` and invoked with `!`.

```rs
@macro
let json!: (TokenStream) -> TokenStream = stream -> {
    let validated := validate_json_tokens(stream);
    transform_to_element_ast(validated)
}

@macro
let sql!: (TokenStream) -> TokenStream = stream -> {
    parse_sql(stream) |> generate_prepared_statement
}

let config := json!{ "host": "localhost", "port": 8080 }
let query  := sql!{ SELECT * FROM users WHERE active = true }
```

If a source file contains no `!` invocations, Phase 1 can be skipped and source text can
stream directly to AST generation.

Token macros cannot be called inside `@comptime` blocks. Cross-phase calls are compile
errors.

### Built-In String Interpolation Macro

String interpolation is a built-in standard macro and requires no import. It runs at
compile time, transforming `{expr}` inside string literals into `<>` concatenation chains.

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

String interpolation macros can be customized, but the general customization API is not
yet specified.

### Custom Macro Syntax

The broader macro design includes domain-specific macros with custom syntax, not only
Element token transformations.

```rs
db |> fetch!(SELECT * FROM users WHERE id = ${user_id})
```

In this form, `${user_id}` captures a scoped variable safely into the domain-specific
syntax. The macro system must eventually support:

1. Declaring token delimiters or trigger syntax
2. Parsing arbitrary syntax up to a defined terminator
3. Reporting domain-specific errors
4. Integrating type safety, such as SQL return schemas

This area is open and should be handled in a dedicated metaprogramming design thread.

### Phase 3: Compile-Time Element Execution (`@comptime`)

`@comptime` elements execute inside the compiler's internal SLUR VM during type checking.
Their return values are substituted directly into the binary.

```rs
@comptime
let generate_op_table: (Arr<string>) -> Arr<(string, int)> = ops -> {
    ops |> map_indexed((op, i) -> (op, i))
}

@comptime
let validate_packet_layout: (Type) -> void = t -> {
    if t.size > 1500 => panic "{t.name} exceeds MTU";
    if t.align != 8  => panic "{t.name} must be 8-byte aligned";
}

@assert validate_packet_layout(UdpPacket)

let @const OPS: Arr<(string, int)> = @run generate_op_table(["add", "sub", "mul"])
```

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
@embed("path/to/file")
@run fn_call()
@insert "other.jz"
@if CONDITION { ... }
@assert CONDITION

let DATA: Arr<u8>  = @embed("assets/icon.png")
let TABLE: Arr<int> = @run build_lookup_table()
```
