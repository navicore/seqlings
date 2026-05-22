# Hint: Square Roots and Pythagoras

The Pythagorean theorem says `c² = a² + b²`, so the hypotenuse is `sqrt(a² + b²)`. Three pieces:

1. Square `a`. Squaring is just a value times itself — `dup f.*`.
2. Square `b`. Same recipe, but `b` is currently UNDER your squared-`a` result. A `swap` brings `b` to the top before the second `dup f.*`.
3. Add the two squares and take the square root.

The interesting bit is step 2's juggling — you compute one square, then reach back for the other operand without losing the first result.

## Squaring without `f.pow`

You could write `2.0 f.pow` instead of `dup f.*`, but the `dup` idiom is shorter and the JIT generally generates tighter code for it. Squaring with `dup f.*` shows up everywhere in numeric code — worth recognizing as a pattern.
