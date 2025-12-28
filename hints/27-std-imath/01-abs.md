# Hint: Absolute Value

## Solution

```seq
: abs-diff ( Int Int -- Int )
    i.subtract imath.abs
;
```

Subtract the two numbers, then take the absolute value of the result.
