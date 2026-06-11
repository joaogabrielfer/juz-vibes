## 15. Operator System

All operators in Element are functions. Operators are distinguished by fixity declaration,
binding rules, associativity, and precedence.

### Fixity - Three Independent Concerns

**`@position`** declares where the symbol sits relative to its arguments:

- `infix` - symbol between two args: `a + b`
- `prefix` - symbol before arg or args: `-a`, `!b`
- `postfix` - symbol after arg or args: `a?`, or RPL-style `a b +`

**`@bind`** maps positional sources to function parameters:

```rs
@bind(left->a, right->b)
@bind(right->a, left->b)
@bind(left(1)->a, left(2)->b)
@bind(right->a)
@bind(left->a)
```

**`@assoc`** declares how chains of the same operator group:

- `left` - `a + b + c` is `(a + b) + c`
- `right` - `a $ b $ c` is `a $ (b $ c)`

### Defining an Operator

```rs
@operator
let @position(infix) @bind(left->a, right->b) @assoc(left) @precedence(6)
(+): (MyNum, MyNum) -> MyNum = { MyNum(a.val + b.val) }

@operator
let @position(postfix) @bind(left(1)->a, left(2)->b) @assoc(left) @precedence(6)
(+): (MyNum, MyNum) -> MyNum = { MyNum(a.val + b.val) }

@operator
let @position(prefix) @bind(right->a) @precedence(9)
(-): (MyNum) -> MyNum = { MyNum(-a) }

@operator
let @position(postfix) @bind(left->a) @precedence(3)
(!): (Arr<T>) -> bool = { a.length == 0 }
```

### Backtick Infix

Any two-argument function can be used as an infix operator by wrapping it in backticks.
It inherits the function's declared `@precedence` if present, otherwise it defaults to
level 4.

```rs
let result := a `compare_by` key
let groups := xs `partition` is_even
let merged := list_a `zip_with` list_b
```

### `~` Context

`~` in expression position is bitwise NOT. In a lambda argument list, left of `->`, `~`
marks the unnamed flowing argument. See [[9-function-syntax-lambdas]].

```rs
let mask := ~flags
let negate_all = ~ -> map(negate)
```

### Master Precedence Table

| Level | Operators | Assoc | Description |
|---|---|---|---|
| 10 | `id`, `echo`, `panic`, `cast`, `todo` | Right | Parenthesis-free prefix intrinsics |
| 9 | `-`, `!`, `~` | Right | Unary negation, logical NOT, bitwise NOT |
| 8 | `#` | Right | N-times function application |
| 7 | `*`, `/`, `%` | Left | Multiplicative arithmetic |
| 6 | `+`, `-`, `<>` | Left | Additive arithmetic and monoid append |
| 5 | `<<<.`, `>>>.` | Left | Bitwise left and right shifts |
| 4 | `>>`, `<<` | Left | Function composition |
| 4 | `` `f` `` | Dynamic | Backtick-wrapped binary function |
| 4 | `<$>`, `<*>` | Left | Functor map and applicative apply |
| 3 | `<`, `>`, `<=`, `>=`, `==`, `!=` | Left | Relational comparisons |
| 2 | `&&`, `||`, `<|>` | Left | Logical operators and alternative |
| 1 | `|>?` | Left | Monadic dependent pipeline |
| 0 | `|>`, `|>>`, `|>>>`, `|>>>>` | Left | Positional pipes |
| 0 | `|~` | Left | Placeholder pipe |
| -1 | `$`, `$$`, `$$$` | Right | Positional right-to-left application |
| -1 | `$~` | Right | Placeholder right-to-left application |
| -2 | `?` | N/A | Postfix early escape / error handler |
