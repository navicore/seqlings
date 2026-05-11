# Float Math

Floating-point math in Seq is split across two layers:

- **Builtins.** Roots, powers, logs, trig, rounding, and the
  constants π / e / τ are *built into the compiler*. They are
  always available — no `include` line, no namespace prefix
  beyond the `f.` that marks them as float ops.

- **`std:fmath`.** Composed helpers — `f.abs`, `f.neg`,
  `f.sign`, `f.square`, `f.clamp`, `f.min`, `f.max` — live in
  the standard library. Bring them into scope with
  `include std:fmath` when you need them.

The exercises in this chapter mostly use the builtins. The
combine exercise pulls in `std:fmath` for `f.abs`.

## Builtin reference

| Group | Words |
|-------|-------|
| Roots / powers | `f.sqrt`, `f.cbrt`, `f.pow` |
| Exp / log | `f.exp`, `f.ln`, `f.log10`, `f.log2` |
| Trigonometric | `f.sin`, `f.cos`, `f.tan`, `f.asin`, `f.acos`, `f.atan`, `f.atan2` |
| Rounding | `f.floor`, `f.ceil`, `f.round`, `f.trunc` |
| Constants | `f.pi`, `f.e`, `f.tau` |

### Notes on semantics

- **Errors are IEEE 754 values, not flags.** `f.sqrt` of a
  negative is `NaN`; `f.ln 0.0` is `-Infinity`. There's no
  `(value Bool)` success flag.
- **`f.round` is banker's rounding** (ties to even, IEEE 754
  default). `0.5 f.round → 0.0`, `1.5 f.round → 2.0`,
  `2.5 f.round → 2.0`. Use `f.trunc`, `f.floor`, or `f.ceil`
  if you need a different rule.
- **`f.atan2` is `( y x -- result )`** — same order as C/Rust/JS.
- **`f.pow` is `( base exp -- result )`.**

## std:fmath reference

```seq
include std:fmath

-3.14 f.abs           # 3.14
2.5 3.7 f.max         # 3.7
3.0 0.0 1.0 f.clamp   # 1.0 (clamped to max)
```

## Exercises in This Section

1. **01-sqrt** — `f.sqrt` and the Pythagorean theorem
2. **02-trig** — `f.sin`, `f.cos`, `f.tan`, and `f.pi`
3. **03-exp-log** — `f.exp`, `f.ln`, `f.log10`, `f.pow`
4. **04-round** — `f.floor`, `f.ceil`, `f.round`, `f.trunc`
5. **05-combine** — combine the above into useful formulas
