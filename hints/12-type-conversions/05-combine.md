# Hint: Chaining Conversions

The pipeline for `string-to-quarter`:

1. Parse the string as an integer with `string->int`. **Watch out:** this returns `( Int Bool )` — the parsed value and a success Bool. You need to drop the Bool before continuing, or the type checker will complain about the next operation seeing the wrong stack shape.
2. Promote that integer to a float with `int->float`.
3. Divide by `4.0` using `f./`.
4. Demote back to int with `float->int`. Truncation discards any fractional part.

The same Bool-drop trap applies to `test-round-trip`: `int->string` returns just a String, but parsing the result back with `string->int` again returns `( Int Bool )`, so the final `drop` between `string->int` and the assertion is needed there too.

## Round-trip considerations

Integer → string → integer round-trips preserve the value exactly. Float → string → float round-trips can lose precision because the string format doesn't always carry every bit of the float (e.g., `0.1` printed and re-parsed may not return to the same bits). Integers, by contrast, have an exact decimal representation that round-trips faithfully.
