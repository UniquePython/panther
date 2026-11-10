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

