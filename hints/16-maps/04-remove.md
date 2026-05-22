# Hint: Removing Entries

Two new map words:

- `map.remove ( Map Key -- Map )` — returns the map with that key gone. If the key wasn't there, returns the map unchanged.
- `map.has? ( Map Key -- Bool )` — returns true if the key is present.

For **`test-remove`**: push the key to remove and call `map.remove`. The do-not-edit assertion afterwards looks up that same key with `map.has?` and expects the answer to be false.

For **`test-contains`**: push the key to check and call `map.has?`, leaving the Bool on the stack for the assertion. Replace the `drop false` placeholder.

## Asserting on Bools

`test.assert` asserts the Bool on top is `true`; `test.assert-not` asserts it is `false`. `test.assert-eq` is integers-only — when an assertion uses it, you know the test produces an Int, not a Bool. When you see `string.equal?` or `map.has?` in front of `test.assert`, that's because those words produce Bools.
