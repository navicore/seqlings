# Hint: roll

`roll` rotates n elements, bringing the deepest one to the top.

## Understanding roll

```seq
1 2 3 4  2 roll    # Rotates top 2: swap       Stack: ( 1 2 4 3 )
1 2 3 4  3 roll    # Rotates top 3: rot        Stack: ( 1 3 4 2 )
1 2 3 4  4 roll    # Rotates top 4             Stack: ( 2 3 4 1 )
```

## Generalizing Patterns

`roll` is a **generalization** of `swap` (2 roll) and `rot` (3 roll). When you see patterns that vary only in a number, you've found an opportunity for generalization.

This is the same insight behind:
- Loops instead of repeated code
- Functions with parameters instead of hardcoded values
- Generic types instead of specific types

## The Solution

The stack is `( 10 20 30 40 )`. You want to bring 10 to the top.

Count the elements: 10 is 4th from top (or "4 elements need to rotate").

```seq
10 20 30 40
4 roll
```
