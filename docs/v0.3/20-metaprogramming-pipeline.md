## 20. Metaprogramming Pipeline

Element splits compilation into isolated, predictable processing phases. Each phase has
strict boundaries preventing information leakage backward in the pipeline.

### Phase 1: Token Macros (`@macro`)

Token macros run on raw, unparsed token streams before the AST is constructed. They are
declared with the `@macro` declaration-level attribute and invoked with the `!` suffix:

```rs
@macro
let json!: (TokenStream) -> TokenStream = stream -> {
    let validated := validate_json_tokens(stream);
    transform_to_element_ast(validated)
};

@macro
let sql!: (TokenStream) -> TokenStream = stream -> {
    parse_sql(stream) |> generate_prepared_statement
};

// Usage at call sites:
let config := json!{ "host": "localhost", "port": 8080 };
let query  := sql!{ SELECT * FROM users WHERE active = true };
```

**Fast-path optimization**: if a source file contains no `!` invocations, Phase 1 is
skipped entirely and source text streams directly to the AST generator.

**Isolation boundary**: token macros cannot be called inside `@comptime` blocks.
Cross-phase calls are a compile error.

### Phase 3: Compile-Time Element Execution (`@comptime`)

`@comptime` elements execute entirely inside the compiler's internal SLUR VM during type
checking. Their return values are substituted directly into the binary:

```rs
@comptime
let generate_op_table: (Arr<string>) -> Arr<(string, int)> = ops -> {
    ops |> map_indexed((op, i) -> (op, i))
};

@comptime
let validate_packet_layout: (Type) -> void = t -> {
    if t.size > 1500 => panic format("{} exceeds MTU", t.name);
    if t.align != 8  => panic format("{} must be 8-byte aligned", t.name);
};

// Invoked at compile time:
@assert validate_packet_layout(UdpPacket);

let @const OPS: Arr<(string, int)> = @run generate_op_table(["add", "sub", "mul"]);
```

### `@auto_impl` — Derived Trait Implementations

A built-in `@macro`-based mechanism for automatically generating `extend` blocks for
type traits with mechanical field mappings:

```rs
// Derive Monoid with explicit field values:
@auto_impl(Monoid(empty: 0, combine: (+)))
def Counter = prod { count: int };

// Derive Eq and Ord using existing functions:
@auto_impl(Eq(equals: (==)), Ord(compare: int_compare))
def Score = prod { value: int };

// Derive Display with a format string:
@auto_impl(Display(show: "Score({})" % self.value))
def Score = prod { value: int };
```

### Statement-Level Directives (Summary)

```rs
@embed("path/to/file")     // embed file contents as Arr<u8>
@run fn_call()              // execute function at compile time
@insert "other.elem"        // textual inclusion
@if CONDITION { ... }       // conditional compilation
@assert CONDITION           // compile-time assertion

// Right-side forms:
let DATA: Arr<u8> = @embed("assets/icon.png");
let TABLE: Arr<int> = @run build_lookup_table();
```

---

