# Panther — Language Semantics (v0.1 — COMPLETE)

## Core identity
- Expression-oriented (almost everything is an expression that produces a value; minimal/no statement-only constructs)
- Statically typed
- Syntax: open, custom-designed (not deliberately copying an existing family)
- v0.1 scope: values, bindings, operators, blocks, branching, loops. No functions, no composite/collection types, no I/O. These are deliberately deferred to v0.2+.
- Also out of scope (future milestones): modules, stdlib, exceptions/results, concurrency, `for`-style iteration (needs collections).

## Primitive types (v0.1)
- `Int`: signed 64-bit integer. Literal is a non-negative digit sequence (leading zeros allowed, e.g. `007` is valid and equals `7`). Negative values are produced via the unary minus operator applied to a non-negative `Int` literal/expression — there is no negative-literal token.
- `Bool`: `true` / `false` keyword literals.
- `Unit`: the "no meaningful value" type. Literal/keyword: `unit`. Exists from the start because the language is expression-oriented — constructs like assignment or loops need *some* value to evaluate to, and `Unit` is that value.

## Variable shadowing (v0.1)
- Redeclaring a name that is already bound **in the same (innermost) scope** is a compile-time error.
- Declaring a name that already exists in an **outer** (enclosing) scope is allowed, and shadows the outer binding for the remainder of the inner scope. The outer binding is restored once the inner scope ends.

## Identifier casing (v0.1)
Identifiers are split into two syntactic classes by first character:
- **UpperIdentifier** (starts with an uppercase letter): used exclusively for type names — `Int`, `Bool`, `Unit` are instances of this class, not hardcoded keywords.
- **lowerIdentifier** (starts with a lowercase letter or _): used exclusively for variable names (and will extend to function names in v0.2+). This is enforced grammatically, not just by convention: an uppercase name can never be used as a variable, and a lowercase name can never be used as a type.

## Variable binding (v0.1)
- Introduced with `decl`. Form: `decl (mut | const) <name> : <Type> = <expr>` — this whole thing is itself an expression.
- Mutability is mandatory and explicit: must write `mut` or `const`, no implicit default.
- Type annotation is mandatory (no inference at binding sites in v0.1).
- The `decl ... = <expr>` expression evaluates to the value just bound (not `Unit`), so bindings can in principle be nested/chained as expressions.
- `decl` is a standalone binding expression (not ML-style `let ... in ...`); it does NOT carry its own sub-scope. Scoping is therefore determined by whatever enclosing block the `decl` sits in — blocks are not yet designed (upcoming decision point).
- Referencing a bound name is itself an expression that evaluates to its current value.

## General syntax rule: expression termination
- `;` is the general terminator used when sequencing expressions one after another (expression-statement separator). This is a language-wide rule, not specific to `decl`.

## Operators (v0.1)

### Arithmetic (on `Int`, produce `Int`)
- `+  -  *  /  %`
- `/` truncates toward zero (C/Java/Rust-style). `-7 / 2 == -3`.
- `%` is truncating remainder, sign follows the dividend. `-7 % 3 == -1`.

### Comparison
- `==` / `!=` (and their desugared/normalized forms) work on **any of the three
  primitive types** (`Int`, `Bool`, `Unit`), as long as both operands are the
  same type as each other. Comparing two `Unit`s is always `true`.
- `< > <= >=` (and their desugared/normalized forms) remain **`Int`-only**.

### Comparison (on `Int`, produce `Bool`)
- Standard: `== != < > <= >=`
- Negated forms (distinct operators, not just sugar removed): `!< !> !<= !>=`
  - `!<`  ≡ `>=` in truth table (not less-than)
  - `!>`  ≡ `<=` in truth table (not greater-than)
  - `!<=` ≡ `>`  in truth table (not less-or-equal)
  - `!>=` ≡ `<`  in truth table (not greater-or-equal)
  - These are intentionally redundant alternate spellings — both forms exist as distinct valid syntax.
- Comparison is **non-associative**: `a < b < c` is a parse error (chaining not supported in v0.1).

### Logical (on `Bool`, produce `Bool`)
- `&&`, `||`, unary `!`
- `&&` and `||` short-circuit.

### Precedence (tightest to loosest)
1. Unary `!`, unary `-`
2. `* / %`
3. `+ -`
4. All comparisons (`< > <= >= == != !< !> !<= !>=`) — same tier, non-associative
5. `&&`
6. `||`

## AST normalization (implementation note, v0.1)

The AST is a deliberately minimal, normalized core — syntax that is semantically
redundant is desugared away at parse time rather than represented as distinct
AST nodes. This keeps all later passes (interpreter, future typechecker) simpler,
since they only ever handle the reduced primitive set.

**Comparisons**: only `Lt` (`<`) and `Eq` (`==`) exist as primitive binary
comparison ops in the AST. All other comparison spellings desugar via `Not`
(unary `!`) and/or swapping operands:

| Surface syntax | AST form        |
|-----------------|------------------|
| `a > b`         | `Lt(b, a)`       |
| `a <= b`        | `Not(Lt(b, a))`  |
| `a >= b`        | `Not(Lt(a, b))`  |
| `a != b`        | `Not(Eq(a, b))`  |
| `a !< b`        | `Not(Lt(a, b))`  |
| `a !> b`        | `Not(Lt(b, a))`  |
| `a !<= b`        | `Lt(b, a)`       |
| `a !>= b`        | `Lt(a, b)`       |

**`unless`**: `unless cond { A } else { B }` desugars to `if` with a negated
condition — there is no `Unless` AST node. This directly reflects `unless`'s
original semantic definition (full drop-in negated `if`), just applied at
parse time instead of carried into the tree.

**Not desugared** (kept as primitive AST nodes, for genuine semantic reasons
rather than just syntactic convenience):
- `&&` / `||` — short-circuit evaluation order is a real semantic that doesn't
  reduce to other primitives without relocating the same complexity elsewhere
  (e.g. into a desugared `if`); kept as dedicated ops.
- Binary `-` — not desugared into `+` and unary negation; kept as its own
  primitive op.
- `%` — kept as its own primitive op; not reconstructed from `/`, `*`, `-`.

Note: this normalization is a *parser/AST-shape* decision, not a change to
Panther's surface syntax or grammar — all ten comparison operators, and both
`if` and `unless`, remain fully valid, distinct syntax for the programmer.
Only their internal tree representation is unified.

## The `Never` type (implementation note, v0.1)
- `break` and `continue` do not have a normal value-producing type (`Unit` would
  be inaccurate — they don't evaluate to a value, they transfer control elsewhere).
  Their type is a special internal type, `Never`.
- `Never` is **not user-referenceable**: it is not registered under any name in the
  type scope (unlike `Int`/`Bool`/`Unit`, which users can write directly). There is
  no surface syntax that produces or names `Never`; it exists purely so the
  typechecker's unification logic works correctly for `break`/`continue`.
- **Unification rule**: wherever two types must "agree" (an `if`'s two branches,
  a `decl`'s annotation vs. its value's type, an assignment's target type vs. the
  assigned value's type), a `Never` actual type always satisfies whatever was
  expected — because if control reaches a `Never`-typed expression, execution
  diverges (breaks/continues) and the surrounding expression's value is never
  actually produced, so there is nothing for the type check to meaningfully object to.
  - `if cond { 5 } else { break }` → type `Int` (the non-`Never` branch wins).
  - `if cond { break } else { continue }` → type `Never` (both branches diverge).
  - `decl const x: Int = if cond { break } else { 5 };` → type-checks fine.
  - `some_var := if cond { break } else { continue };` → type-checks fine,
    regardless of `some_var`'s declared type, since the assignment never
    actually executes on either path.
- `Never` is only ever special as an **actual** type; nothing in v0.1 ever
  expects a value of type `Never`.

## Blocks (v0.1)
- Delimited with braces: `{ ... }`.
- A block is a `;`-separated sequence of expressions.
- **Value rule**: if the block ends with an expression that has NO trailing `;`, the block evaluates to that expression's value. If every expression in the block is `;`-terminated (including the empty block `{ }`), the block evaluates to `Unit`.
  - e.g. `{ decl const x: Int = 5; x }` → evaluates to `5`
  - e.g. `{ decl const x: Int = 5; x; }` → evaluates to `Unit`
  - e.g. `{ }` → evaluates to `Unit`
- A block is itself an expression and can appear anywhere an expression can (e.g. nested, or as the value of a `decl`).

## Control flow: `if` / `unless` / `when` (v0.1)
- No parentheses around the condition in any of these.
- **`if cond { ... } else { ... }`** — value-producing conditional expression.
  - `else` is mandatory (no optional-else form of `if`).
  - Both branches must type-check to exactly the same type (no unification/subtyping in v0.1); the `if` expression's type is that shared type.
  - `else if cond { ... }` chaining is supported; no separate `elif`/`elseif` keyword.
- **`unless cond { ... } else { ... }`** — full drop-in replacement for `if`, meaning `if !cond { ... } else { ... }`. Same mandatory-else and same-type rules apply. `unless` does NOT apply to `when`.
- **`when cond { ... }`** — one-sided conditional, no `else` form exists at all. Always evaluates to `Unit` (consistent with a block with no unmatched branch to unify against — same rule as a `;`-terminated/empty block).

## Loops (v0.1)
- **`while cond { ... }`** — standard condition-checked loop, runs while `cond` is `true`.
- **`repeat count { ... }`** — `count` is an `Int`-valued expression, evaluated once up front; body runs `count` times.
  - `count == 0` → runs zero times, no error.
  - `count < 0` → **runtime error** (not silently treated as zero, to avoid hiding bugs).
- Both loop forms always evaluate to `Unit`, regardless of how many iterations ran (including zero).
- `break` and `continue` exist, carry no value (consistent with loops always being `Unit`).
  - Valid only inside a loop body; using either outside any enclosing loop is an error (checked statically/at compile time, not a grammar-level restriction).
  - No `for`-style iteration yet — deferred until collections/ranges are designed.

## Assignment (v0.1)
- Only valid on names declared with `mut`. Assigning to a `const`-declared name is a **compile-time error**.
- Two assignment operators, distinguished by which value the assignment expression evaluates to:
  - `x := expr` — assigns, evaluates to the **previous (old) value** of `x`.
  - `x =: expr` — assigns, evaluates to the **newly assigned value**.
- No compound assignment operators (`+=` etc.) in v0.1 — deferred; trivial to add later as sugar.

