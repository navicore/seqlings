# Hint: Trigonometric Functions

Both tests follow the shape of `test-sin-zero` (already filled in): push the input, apply the trig function, the assertion follows.

- **`test-cos-zero`** — cosine of zero. Same shape as the existing sin test with one word changed.
- **`test-sin-pi-half`** — sine of π/2. The angle isn't a literal you can push directly; compute it from `f.pi`. Dividing the constant by 2.0 gives you π/2, then apply sine.

`f.pi`, `f.sin`, `f.cos`, `f.tan` (and their inverses `f.asin`, `f.acos`, `f.atan`) are all builtins — no `include` needed.

## Why radians?

Calculus likes radians: the derivative of `sin(x)` is `cos(x)` only when x is in radians. So every standard library uses radians, and any time you have degrees you convert first. The next exercise sets you up to write that conversion.
