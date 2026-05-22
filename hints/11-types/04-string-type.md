# Hint: String Operations

The four tests each need exactly one `string.*` operator, picked from the menu the exercise prose lists. The pattern is the same as the int/float-ops exercises: push the literal(s), apply the operator, the assertion follows.

The vocabulary you'll reach for:

- **Length** — counts characters in one string.
- **Concat** — joins two strings into one.
- **Empty?** — asks "is this string the empty string?" Returns a Bool, no value to discard.
- **Equal?** — asks "are these two strings the same?" Useful for asserts because `test.assert-eq` doesn't work on strings (it's Int-only).

The `?` suffix on `empty?` / `equal?` follows the predicate convention from chapter 08 — those words return Bool. Words without `?` return the actual computed value (a length, a new string).

Two of these tests assert the result is true, two assert the result is `not true`. Read the assertion line first; it tells you what shape result the body needs to produce.
