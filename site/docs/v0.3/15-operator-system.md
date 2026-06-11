## 15. Operator System

All operators in Element are functions. `a + b` desugars to `add(a, b)`. There is no
special-cased operator syntax at the language level. Operators are distinguished purely
by their fixity declaration.

### Fixity — Three Independent Concerns

**`@position`** — where the symbol sits relative to its arguments:
- `infix`   — symbol between two args: `a + b`
- `prefix`  — symbol before arg(s): `-a`, `!b`
- `postfix` — symbol after arg(s): `a?`, or RPL-style `a b +`

**`@bind`** — maps positional sources to function parameters:

```rs
@bind(left->a, right->b)        // infix standard: left=a, right=b
@bind(right->a, left->b)        // infix flipped
@bind(left(1)->a, left(2)->b)   // binary postfix / RPL stack style
@bind(right->a)                 // prefix unary
@bind(left->a)                  // postfix unary
```

**`@assoc`** — how chains of the same operator group:
- `left`  — `a + b + c` = `(a + b) + c`
- `right` — `a $ b $ c` = `a $ (b $ c)`

### Defining an Operator

```rs
// Standard infix addition for a custom type:
@operator
let @position(infix) @bind(left->a, right->b) @assoc(left) @precedence(6)
(+): (MyNum, MyNum) -> MyNum = { MyNum(a.val + b.val) };

// RPL-style postfix binary (both args from the left):
@operator
let @position(postfix) @bind(left(1)->a, left(2)->b) @assoc(left) @precedence(6)
(+): (MyNum, MyNum) -> MyNum = { MyNum(a.val + b.val) };
// Usage: mynum1 mynum2 +

// Prefix negation:
@operator
let @position(prefix) @bind(right->a) @precedence(9)
(-): (MyNum) -> MyNum = { MyNum(-a.val) };

// Custom postfix operator:
@operator
let @position(postfix) @bind(left->a) @precedence(3)
(!): (Arr<T>) -> bool = { a.length == 0 };
// Usage: arr!  — true if empty
```

### Backtick Infix

Any two-argument function can be used as an infix operator by wrapping it in backticks.
Inherits the function's declared `@precedence` if present, otherwise defaults to level 4:

```rs
let result := a \compare_by\ key;
let groups := xs \partition\ is_even;
let merged := list_a \zip_with\ list_b;
```

### Master Precedence Table

| Level | Operators | Assoc | Description |
|---|---|---|---|
| **10** | `id`, `echo`, `panic`, `cast`, `todo` | Right | Parenthesis-free prefix intrinsics |
| **9** | `-`, `!`, `~` | Right | Unary negation, logical NOT, bitwise NOT |
| **8** | `#` | Right | N-times function application |
| **7** | `*`, `/`, `%` | Left | Multiplicative arithmetic |
| **6** | `+`, `-`, `<>` | Left | Additive arithmetic and monoid append |
| **5** | `<<<.`, `>>>.` | Left | Bitwise left and right shifts |
| **4** | `>>`, `<<` | Left | Left-to-right and right-to-left function composition |
| **4** | `` \f\ `` | Dynamic | Backtick-wrapped binary function |
| **4** | `<$>`, `<*>` | Left | Functor map and applicative apply |
| **3** | `<`, `>`, `<=`, `>=`, `==`, `!=` | Left | Relational comparisons |
| **2** | `&&`, `\|\|`, `<\|>` | Left | Logical operators and alternative |
| **1** | `\|>?` | Left | Monadic dependent pipeline |
| **0** | `\|>`, `\|>>`, `\|>>>`, `\|>>>>` | Left | Positional pipes |
| **0** | `\|~` | Left | Placeholder pipe |
| **-1** | `$`, `$$`, `$$$` | Right | Positional right-to-left application |
| **-1** | `$~` | Right | Placeholder right-to-left application |
| **-2** | `?` | N/A | Postfix early escape / error handler |

---

