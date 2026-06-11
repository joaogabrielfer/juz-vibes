## 15. Notation System

All notations in Element are functions. Symbolic forms such as `(+)` and named forms such
as `(echo)` use the same declaration model.

### Notation Declaration

`@notation` marks a declaration as notation-enabled. Configuration attributes must be on a
single line above the `let` declaration.

```rs
@notation
@position(infix) @bind(left->a, right->b) @assoc(left) @precedence(6) @adjacent(left, right)
let (+): (MyNum, MyNum) -> MyNum = { MyNum(a.val + b.val) }

@notation
@position(prefix) @bind(right->x) @precedence(10) @adjacent(right)
let (echo): <T: Display> (T) -> () = { vm.echo(x.show) }

@notation
@position(postfix) @bind(left(1)->a, left(2)->b) @assoc(left) @precedence(6) @adjacent(left)
let (+): (MyNum, MyNum) -> MyNum = { MyNum(a.val + b.val) }
```

### Position, Binding, and Grouping

- `@position(infix)` places the notation between arguments.
- `@position(prefix)` places the notation before its arguments.
- `@position(postfix)` places the notation after its arguments.
- `@bind(...)` maps positional operand sources to parameter names.
- `@assoc(left/right)` controls grouping for repeated notation chains.

Examples:

```rs
@bind(left->a, right->b)
@bind(right->a)
@bind(left->a)
@bind(left(1)->a, left(2)->b)
```

### Adjacency and Tokenization

`@adjacent(...)` permits no-whitespace adjacency on the selected bound sides:

- `@adjacent(left)`
- `@adjacent(right)`
- `@adjacent(left, right)`

Tokenization is maximal-munch and declaration-agnostic. A notation declaration never
splits identifier tokens.

```rs
echofoo         // one identifier token, never `echo foo`
echo_all_values // one identifier token
```

Valid adjacency examples:

```rs
1+2
echo"hello"
echo-1
echo~flags
```

### Named Prefix Notation

Named notations such as `echo`, `panic`, `todo`, and `cast` are normal notation
declarations. They are not special parser forms.

```rs
echo value
panic "bad state"
cast<int>(value)
```

Bare prefix notation is not general whitespace application. When chaining application
steps, use `$` explicitly:

```rs
foo $ bar $ baz
```

### Backtick Infix

Any two-argument function can be used as infix notation by wrapping it in backticks. It
inherits the function's `@precedence` if present, otherwise defaults to level 4.

```rs
let result := a `compare_by` key
let groups := xs `partition` is_even
let merged := list_a `zip_with` list_b
```

### `~` Context

`~` has three context-dependent roles:

- Expression position: bitwise NOT (`~flags`)
- Lambda argument list, left of `->`: unnamed flowing argument
- Inside `|~` and `$~`: pipe/reverse-application placeholder

The parser distinguishes these by syntactic context.

### Core Precedence Table

Imported functional notations such as '>>=', `<$>`, and `<*>` are specified in
[[16-std-functional]].

| Level | Notations | Assoc | Description |
|---|---|---|---|
| 10 | `echo`, `panic`, `cast`, `todo` | Right | Named prefix notation (typically unary) |
| 9 | `-`, `!`, `~` | Right | Unary negation, logical NOT, bitwise NOT |
| 7 | `*`, `/`, `%` | Left | Multiplicative arithmetic |
| 6 | `+`, `-`, `<>` | Left | Additive arithmetic and append |
| 5 | `<<<.`, `>>>.` | Left | Bitwise left and right shifts |
| 3 | `<`, `>`, `<=`, `>=`, `==`, `!=` | Left | Relational comparisons |
| 2 | `&&`, `\|\|` | Left | Logical operators |
| 0 | `\|>`, `\|>>`, `\|>>>`, `\|>>>>`, `\|~`, `\|>?` | Left | Pipe family |
| -1 | `$`, `$$`, `$$$`, `$~` | Right | Reverse application family |
| -2 | `?` | N/A | Postfix early escape / error handler |
