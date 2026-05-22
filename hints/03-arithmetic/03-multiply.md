# Hint: i.*

`i.*` multiplies two integers and leaves the product on the stack. Stack effect: `( a b -- product )`.

Push both operands and apply `i.*`. The test wants 60, so pick operands that multiply to 60. Multiplication is commutative, so either order works.

## The `i.` prefix

Like every integer operator (`i.+`, `i.-`, `i./`), `i.*` lives in the `i.` family — one of the type-prefix conventions you saw in chapter 11. Seq's type-inferred `*` also works when the operand types are unambiguous, but the explicit `i.*` is universal.
