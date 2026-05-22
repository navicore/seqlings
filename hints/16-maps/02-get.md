# Hint: Getting Values

`map.get` has stack effect `( Map Key -- Value Bool )` — it returns the value AND a Bool indicating whether the key was found. When you know the key exists (as in these tests), discard the Bool with `drop` and proceed with the value.

The two tests are identical in shape — only the key differs. Each body is three tokens: push the key, call `map.get`, drop the Bool. The `string.equal?` assertion that follows expects the value on top.

## Why two return values?

A lookup that returned a "not found" sentinel (like `null` in many languages) would force callers to pick a value that can never be real — a design that breaks the moment you want to intentionally store nulls. Returning the Bool separately keeps the type uniform and makes the success/failure question explicit. The next exercise teaches the safe branching pattern that actually uses the Bool.

For these tests, since the keys definitely exist, `drop` is the right call.

## Why `string.equal?` and not `test.assert-eq`?

`test.assert-eq` is integers-only. For strings, you compute the comparison Bool yourself with `string.equal?` and then assert it with `test.assert`. That's the do-not-edit line.
