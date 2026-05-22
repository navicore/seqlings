# Hint: Safe Access

There's no `map.get-or` builtin — you build it from `map.get` and an `if` that branches on the found-Bool.

The plumbing challenge: arguments arrive as `( Map Key Default )` — Default on top. But `map.get` wants `( Map Key )` on top. The Default needs to be set aside FIRST, so `map.get`'s result lands above it with the Bool on top ready for `if`.

A `rot rot` (two rotations of three items) buries the Default below Map and Key. Then `map.get` lands `( Default Value Bool )` on the stack: Default at the bottom, the looked-up Value above it, the Bool on top.

The `if` then chooses:

- **Found branch**: stack is `( Default Value )` — keep Value, drop Default. The word that drops the SECOND item is what you want.
- **Missing branch**: stack is `( Default placeholder )` — drop the placeholder (it's garbage), keep Default. Plain `drop` does that.

The two branch quotations are one token each. Picking which goes where is the reasoning step the exercise wants from you.

## Why this pattern is everywhere

Almost every dictionary/map lookup in real code reaches for the same "got-it OR fallback" shape: HTTP request headers, config files, query string params. Languages that don't return a found-Bool force you to use sentinels (`null`, `undefined`, `None`), and every consumer has to remember to check. The two-value return makes the check unavoidable.
