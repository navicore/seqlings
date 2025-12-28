# Hint: Clamping Values

## Solution

```seq
: clamp-percent ( Int -- Int )
    0 100 imath.clamp
;
```

The clamp function takes: value, min, max and returns the clamped value.
