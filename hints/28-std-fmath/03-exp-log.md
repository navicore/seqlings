# Hint: Exponential and Logarithmic

The most useful thing to know here: **`f.pow` argument order is `base exp`.** So 2⁸ pushes 2 first, then 8, then calls `f.pow`. This trips people up — many language libraries write `pow(b, e)` and you have to remember which side is which.

For `test-log10`, the test value (100.0) is already a clean power of ten — push it and call the log-base-10 builtin.

The full menu, all builtins (no `include` needed):

- `f.exp ( Float -- Float )` — eˣ
- `f.ln ( Float -- Float )` — natural log
- `f.log10 ( Float -- Float )` — log base 10
- `f.log2 ( Float -- Float )` — log base 2
- `f.pow ( Float Float -- Float )` — base^exp

Errors are IEEE 754 values: `0.0 f.ln` produces `-Infinity`, a negative produces `NaN`. No flag to check.
