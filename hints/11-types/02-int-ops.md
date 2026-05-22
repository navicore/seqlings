# Hint: Integer Operations

The shape for each test is the same: push the two operands the exercise prose tells you, apply the right operator, the assertion follows.

The interesting thing here is the **Bool-flag trap** on division and modulo:

- `i.+`, `i.-`, `i.*` return just an `Int`.
- `i./` and `i.modulo` return `( Int Bool )` — the quotient/remainder plus a success flag that's `false` only when the divisor is zero.

That's why the two divide tests carry *two* assertions: the first checks the Bool (success), the second checks the value. If you forget the flag is there, your stack will be off by one and the type checker will refuse the file.

The four operator names you'll need are listed in the exercise prose. Match each to its test by what arithmetic it does, and leave the right number of items on the stack for the asserts that follow.
