# Hint: Min and Max

Compute max and min of the same two values, then subtract.

## Solution

```seq
: range ( Int Int -- Int )
    over over max
    rot rot min
    i.-
;
```
