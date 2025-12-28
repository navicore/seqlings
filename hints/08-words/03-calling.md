# Hint: Words Calling Words

x^4 = (x^2)^2 = square(square(x))

## Solution

```seq
: fourth-power ( Int -- Int )
    square square
;
```

That's it! Call square, which squares the number. Call it again on the result, squaring the square.

## The Power of Composition

This is **function composition** in action:
- `square` transforms x → x²
- `square square` transforms x → x² → x⁴

Each word is a building block. Combine them to build bigger blocks.
