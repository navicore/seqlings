# Hint: Equality (i.=)

`i.=` consumes two integers and pushes a Bool — `true` if they're equal, `false` otherwise. Stack effect: `( a b -- Bool )`.

The test asks whether 42 equals 42. Push 42 twice, apply `i.=`, the assertion that follows checks the resulting Bool is `true`.

## A note on `=` vs `i.=`

Seq has a type-inferred `=` that picks the right comparison for its operands (`i.=` for ints, `f.=` for floats). For the rest of this chapter you'll see the explicit `i.` form because the test cases are integers throughout. Either name works for the assertion here.

## Equality as abstraction

Equality testing is a form of abstraction — you're asking "are these the same?" without caring what they actually are. The same question applies to any comparable values, which is why every type has its own equality operator under the type-prefix convention.
