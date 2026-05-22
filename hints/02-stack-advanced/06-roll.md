# Hint: roll

`n roll` rotates `n+1` items, bringing the value at depth `n` (0-indexed from the top) to the top.

So:

- `0 roll` does nothing (rotates 1 item — itself).
- `1 roll` is `swap` (rotates 2 items, brings depth-1 to top).
- `2 roll` is `rot` (rotates 3 items, brings depth-2 to top).
- `3 roll` rotates 4 items, brings depth-3 to top.

## Generalizing patterns

`roll` is a generalization of `swap` (which is `1 roll`) and `rot` (which is `2 roll`). When you see patterns that vary only in a number, you've found an opportunity for generalization. Same insight as:

- Loops instead of repeated code.
- Functions with parameters instead of hardcoded values.
- Generic types instead of specific types.

## Solving this exercise

The starting stack is `( 10 20 30 40 )`. To find the right `n`, count from the top using 0-indexing:

- depth 0 = 40 (top)
- depth 1 = 30
- depth 2 = 20
- depth 3 = 10

You want to bring 10 to the top, so `n` should match its depth.
