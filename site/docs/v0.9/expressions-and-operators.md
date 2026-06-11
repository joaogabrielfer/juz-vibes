# Expressions and Operators

Element has no intrinsic value-level operators. Operator-like syntax is a notation call
surface over ordinary user-level declarations.

The parser owns:

- where a notation may appear
- how operands bind to the notation declaration
- precedence and associativity
- tokenization and adjacency rules

The standard library owns the default declarations for symbols such as `+`, `*`, `==`,
`&&`, `||`, `<>`, `echo`, `panic`, `todo`, and `cast`. User code and packages may define
additional notations through the same declaration model, subject to normal import,
visibility, and coherence rules.

## String Append

`<>` appends strings and other monoidal values. It is parsed as an infix notation and is
provided by the default standard surface.

```rust
let greeting := "Hello " <> name <> "!"
```

String interpolation is a built-in compile-time lowering over this surface. The parser
does not treat `<>` as a primitive operation; it lowers interpolated fragments into
ordinary append and formatting calls.

```rust
println("Hello {name}, count: {count}")
```

## Positional Pipes

The pipe family is part of the core expression grammar because the parser must recognize
argument insertion position.

The left-hand side is passed as the Nth argument to the right-hand function. The number of
`>` symbols minus one indicates which argument position receives the piped value. The
family is capped at four arrows.

```rust
a |>   f(b, c)    // f(a, b, c)
a |>>  f(b, c)    // f(b, a, c)
a |>>> f(b, c)    // f(b, c, a)
a |> bar          // bar(a)

users |> filter(active)
value |>> divide(100.0)
item  |>>> insert(db, "users_table")
```

## Placeholder Pipe

`|~` uses `~` placeholders to mark exactly where piped values land. The number of
placeholders must match the number of piped values.

```rust
x, y    |~ f(~, ~, z, 4)
x, y, z |~ f(0, ~, ~, ~)
value   |~ transform(pre, ~, post)
```

This is a compile error because two values are piped but three placeholders are present:

```rust
x, y |~ f(~, ~, ~, 4)
```

## Value Group Spreading

Spread is written `..` and can be combined with pipes.

```rust
let coords := (10, 20)

coords   |> draw_at(style)    // draw_at(coords, style)
coords.. |> draw_at(style)    // draw_at(10, 20, style)
```

## Fallible Pipeline

`|>?` is the fallible variant of the pipe family. It applies only when the left-hand type
implements `IntoFallible<T>`.

If an error or failure variant is encountered, it stays wrapped and propagates through the
rest of the chain. The outer function continues executing with the wrapped result.

```rust
let report := user_id
    |>  fetch_user
    |>? load_account
    |>? get_balance

log_attempt(report)
report
```

The syntax is core grammar. The carrier and conversion contract are standard-library
surface; see [[std-error]].

## Reverse Application

`$` sits below pipes and is right-associative. It mirrors positional pipe insertion.

- `f $ x` means `f(x)`
- `f(a) $$ x` means `f(a, x)`
- `f(a, b) $$$ x` means `f(a, b, x)`

```rust
echo $ raw_data |> parse_csv |> summarize
divide(100.0) $$ value
insert(db, "users_table") $$$ item
```

`foo $$ bar` is invalid when the left side does not already provide an insertion slot for
the requested argument position.

## Placeholder Reverse Application

`$~` is reverse application with explicit `~` placeholders.

```rust
format $~ "Hello, ~!" $ get_name()
```

## Partial Application

Using `_` in a regular function call, outside pipe placeholder context, creates a closure
waiting for the missing argument or arguments. The resulting closure matches the arity of
the holes.

```rust
let add_one   := add(1, _)
let scale_by  := multiply(_, 10)
let partial   := f(_, "fixed", _)
let divide_by := flip(divide)(2.0, _)

arr |> map(add_one)
arr |> filter(greater(0, _))
```

Inside `|~` and `$~`, `~` is the pipe/reverse-application placeholder and must match the
number of piped values exactly. `_` keeps its normal partial-application meaning.

```rust
foo |~ map(~, add(1, _))
```

## Notation Declarations

All notations are functions. Symbolic forms such as `(+)` and named forms such as
`(echo)` use the same declaration model.

```rust
@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(6) @adjacent(left, right)
let (+): (MyNum, MyNum) -> MyNum = { MyNum(a.val + b.val) }

@notation
@position(prefix) @bind(right->x) @precedence(10) @adjacent(right)
let (echo): <T: Display> (T) -> () = { print Display.show(x) }
```

The call-shape attributes used by notation declarations are also reused by `@macro` and
bang-callable `@comptime` declarations. Reuse of the shape vocabulary does not make those
elements notation declarations. Metaprogramming call rules are specified in
[[metaprogramming]].

## Position, Binding, and Grouping

- `@position(infix)` places the notation between arguments.
- `@position(prefix)` places the notation before its arguments.
- `@position(postfix)` places the notation after its arguments.
- `@bind(...)` maps operand sources to parameter names.
- `@assoc(left/right)` controls grouping for repeated notation chains.

Examples:

```rust
@bind(left->a, right->b)
@bind(right->a)
@bind(left->a)
@bind(left(1)->a, left(2)->b)
```

## Adjacency and Tokenization

`@adjacent(...)` permits no-whitespace adjacency on selected sides:

- `@adjacent(left)`
- `@adjacent(right)`
- `@adjacent(left, right)`

Tokenization is maximal-munch and declaration-agnostic. A notation declaration never
splits identifier tokens.

```rust
echofoo         // one identifier token, never `echo foo`
echo_all_values // one identifier token
```

Valid adjacency examples:

```rust
1+2
echo"hello"
echo-1
echo~flags
```

## Named Prefix Notation

Named notations such as `echo`, `panic`, `todo`, and `cast` are standard declarations,
not parser-only keywords.

```rust
echo value
panic "bad state"
cast<int>(value)
```

Bare prefix notation is not general whitespace application. When chaining application
steps, use `$` explicitly.

```rust
foo $ bar $ baz
```

## Backtick Infix

Any two-argument function can be used as infix notation by wrapping it in backticks. It
inherits the function's `@precedence` if present, otherwise defaults to level 4.

```rust
let result := a `compare_by` key
let groups := xs `partition` is_even
let merged := list_a `zip_with` list_b
```

Handler functions such as `recover` may use the same form:

```rust
let config := load_config(path) `recover` [
    .FileMissing(e) => resume .UsePath(default_config_path)
]
```

## `~` Context

`~` has three context-dependent roles:

- expression position: bitwise NOT notation, such as `~flags`
- lambda argument list, left of `->`: unnamed flowing argument
- inside `|~` and `$~`: pipe/reverse-application placeholder

The parser distinguishes these by syntactic context.

## Core Precedence Table

The table records parse precedence and associativity. It does not imply that these
symbols are intrinsic VM operations. The default declarations are documented in
[[std-prelude]].

| Level | Notations | Assoc | Description |
|---|---|---|---|
| 10 | `echo`, `panic`, `cast`, `todo`, `yield`, `stream`, `collect`, `recover` | Right | Named prefix notation and core handler names |
| 9 | `-`, `!`, `~` | Right | Unary negation, logical NOT, bitwise NOT |
| 7 | `*`, `/`, `%` | Left | Multiplicative arithmetic |
| 6 | `+`, `-`, `<>` | Left | Additive arithmetic and append |
| 5 | `<<<.`, `>>>.` | Left | Bitwise left and right shifts |
| 3 | `<`, `>`, `<=`, `>=`, `==`, `!=` | Left | Relational comparisons |
| 2 | `&&`, `\|\|` | Left | Logical operators |
| 0 | `\|>`, `\|>>`, `\|>>>`, `\|>>>>`, `\|~`, `\|>?` | Left | Pipe family |
| -1 | `$`, `$$`, `$$$`, `$~` | Right | Reverse application family |
| -2 | `?` | N/A | Postfix early escape / local error handler |
