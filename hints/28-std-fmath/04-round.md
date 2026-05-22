# Hint: Rounding

Four rounding modes, each a one-word body:

- `f.floor` — toward −∞.
- `f.ceil` — toward +∞.
- `f.round` — nearest, ties to even (banker's rounding).
- `f.trunc` — toward zero.

Each test pushes its input literal and applies one of these. Pick by matching what the assertion expects:

- **`test-ceil`** expects 4.0 from 3.2 — needs the toward-+∞ word.
- **`test-round`** expects 3.0 from 2.6 — needs nearest.
- **`test-trunc`** expects −3.0 from −3.7 — needs toward-zero.

## A note on `f.round` ties

The test uses 2.6 (not a tie) so banker's rounding doesn't matter. But it would for an input like 2.5 — that rounds to `2.0` (nearest *even*), not `3.0`. If you ever need "always round half up," `f.trunc` after biasing the input is the standard escape hatch.
