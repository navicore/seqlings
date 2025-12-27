# Hint: Push

This exercise teaches you how values are pushed onto the stack.

## Pushing Values

In Seq, writing a value pushes it onto the stack. To push multiple values, just write them one after another:

```seq
1       # Stack: ( 1 )
1 2     # Stack: ( 1 2 )
1 2 3   # Stack: ( 1 2 3 )
```

## Stack Notation

Stack effects are written as `( before -- after )`.

The leftmost value is at the bottom, rightmost is on top:
- `( 1 2 3 )` means 1 is at the bottom, 3 is on top

## The Solution

Replace `0 0 0` with `1 2 3`.

When the test runs, it checks:
1. Pop top, should be 3
2. Pop next, should be 2
3. Pop next, should be 1
