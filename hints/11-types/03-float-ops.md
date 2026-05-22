# Hint: Float Operations

Same shape as the integer-ops exercise — push the two operands the prose specifies, apply the right operator, the assertion follows.

The vocabulary differences worth keeping in mind:

- The prefix is `f.` instead of `i.`. Everything you used for ints has a `f.`-twin: `f.+`, `f.-`, `f.*`, `f./`.
- Unlike `i./`, **`f./` returns only a Float** — no Bool flag. Floats just produce `inf` or `nan` on degenerate inputs instead of failing, so there's nothing to check. The asserts in this exercise reflect that: no `test.assert` before the value check.
- The comparison `f.=` is the right way to ask "are these equal" for floats; using `=` (the Int form) would be a type error.

The bodies are short because the lesson is the prefix family, not the arithmetic. Once you know `f.` is "the float version of," every operation you've seen for ints has its float partner.

## Why `f.=` for equality?

In most languages `==` on floats is a trap (you can have `0.1 + 0.2 != 0.3` due to binary representation). Seq's `f.=` typically does the same exact-bits check, so it has the same gotcha — but at least the name reminds you "I'm doing float equality, here be dragons." In real code you usually want an `abs(a - b) < epsilon` check; for these test cases the math is clean so `f.=` is fine.
