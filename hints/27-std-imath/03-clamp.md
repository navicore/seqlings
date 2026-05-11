# Hint: Clamping Values

`clamp` takes value, min, max and returns the clamped value.

## Solution

```seq
: clamp-percent ( Int -- Int )
    0 100 clamp
;
```
