# Hint: Mutual Recursion

The pattern is parity-flipping: each call decrements n by 1 and hands off to the OTHER function. Eventually n hits 0 and whichever function is active returns its hard-coded answer.

So `is-even?` has the recursion shape you already know, with one twist:

- **Base case** at 0: return `true` (zero is even).
- **Recursive case**: subtract 1, call `is-odd?` instead of itself.

`is-odd?` is the mirror image:

- **Base case** at 0: return `false` (zero isn't odd).
- **Recursive case**: subtract 1, call `is-even?`.

Each body is the exact recursion shape from earlier exercises, just with the recursive call going to the partner instead of itself. No new mechanics — and no `i.+` or `i.*` to combine results, because each function returns a Bool that doesn't need combining.

## How it cascades

- `is-even?(4)` → `is-odd?(3)` → `is-even?(2)` → `is-odd?(1)` → `is-even?(0)` → `true`
- `is-odd?(4)` → `is-even?(3)` → `is-odd?(2)` → `is-even?(1)` → `is-odd?(0)` → `false`

## Real-world mutual recursion

The parity example is the simplest illustration; in real code mutual recursion shows up in:

- **Parsers** — expressions contain terms, terms contain factors, factors contain expressions.
- **State machines** — state A transitions to state B, B transitions back to A.
- **Tree traversals with multiple node types** — `Cons` vs `Empty`, internal node vs leaf.

The shape is the same: two (or more) functions, each shrinking the problem and handing off to the others.
