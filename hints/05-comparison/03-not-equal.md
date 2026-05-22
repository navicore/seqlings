# Hint: Not Equal (i.<>)

`i.<>` is the negation of `i.=`. It consumes two integers and pushes `true` when they're not equal, `false` when they are. Stack effect: `( a b -- Bool )`.

For this test, push the two values from the prose (100 and 99), apply `i.<>`, and the assertion checks the resulting Bool is `true`. Since 100 and 99 aren't equal, the operator returns `true`.

## The `<>` symbol

The `<>` notation comes from mathematics and older programming languages (Pascal, SQL). Read it as "less than OR greater than" — if either is true, the values aren't equal. Modern languages tend to use `!=`, but `<>` survives in code where ASCII-vs-Unicode equality glyphs were a concern.

You could also write `i.= not` and get the same answer. `i.<>` is the dedicated word; `not` flipping the result of `i.=` is the do-it-yourself version.
