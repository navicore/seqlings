# Hint: Integer/Float Conversions

Two new conversion words:

- `int->float ( Int -- Float )` — promotes an integer to its float form. `7` becomes `7.0`.
- `float->int ( Float -- Int )` — demotes a float by **truncating toward zero**. `7.9` becomes `7`; `-2.9` becomes `-2` (not `-3` — toward zero, not toward `-∞`).

The first test is a four-step pipeline: push 7, promote it to float, add 0.5 (with `f.+`, since we're now in float-land), demote back to int. Truncation discards the .5, so the final value is 7.

The second test is given — it's a one-step demonstration that 9.9 truncates to 9.

## Why truncation toward zero?

Different languages pick different rules:

- **Toward zero** (Seq, C, Java): `-2.9 → -2`, `2.9 → 2`. Symmetric around zero.
- **Toward -∞** (Python's `//` on floats, math floor): `-2.9 → -3`, `2.9 → 2`. Always rounds down.

The first is what most CPUs do natively, so it's the default. If you want toward-`-∞`, use `f.floor` before `float->int`.
