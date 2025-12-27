# Hint: rot

The `rot` word rotates the top three elements, bringing the third to the top.

## How rot Works

```seq
1 2 3 rot    # Stack: ( 2 3 1 )
```

Stack effect: `( a b c -- b c a )`

Think of it as: the third element "rotates" up to become the new top.

## The Solution

Starting with `( 3 1 2 )`, you need `( 1 2 3 )`. Add `rot`:

```seq
3 1 2
rot
3 test.assert-eq
2 test.assert-eq
1 test.assert-eq
```

After `rot`, the 3 moves from bottom to top.

## Visualizing rot

Before: `( 3 1 2 )`
         ↓ ↓ ↓
After:  `( 1 2 3 )`

The bottom value (3) moves to the top, and the others shift down.

## Related Words

- `rot` moves third to top: `( a b c -- b c a )`
- `-rot` is the reverse (if available): `( a b c -- c a b )`
