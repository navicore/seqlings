# Hint: Integer Power

## Solution

```seq
: power-of-2 ( Int -- Int )
    2 swap imath.pow
;
```

Put 2 on the stack, swap so exponent is on top, then call pow.
