# Hint: String Length

`longer-than-five?` asks whether a string's character count exceeds 5. Two pieces:

1. Compute the length — `string.length` consumes the string and pushes an Int.
2. Compare that Int to 5 — push 5, then a comparison operator.

That's the entire word body. The `?` suffix on the name signals "this returns a Bool" — match the stack-effect signature `( String -- Bool )` accordingly.
