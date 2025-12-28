# Hint: Min and Max

## Solution

```seq
: range ( Int Int -- Int )
    2dup imath.max    # Get max
    rot rot imath.min # Get min
    i.-               # max - min
;
```

Or more simply using over:

```seq
: range ( Int Int -- Int )
    over over imath.max
    rot rot imath.min
    i.-
;
```
