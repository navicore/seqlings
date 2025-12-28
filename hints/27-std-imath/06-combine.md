# Hint: Combining Math Operations

## Manhattan Distance

```seq
: manhattan-distance ( Int Int Int Int -- Int )
    # Stack: x1 y1 x2 y2
    rot i.- imath.abs   # y-diff: |y2-y1|
    rot rot i.- imath.abs  # x-diff: |x2-x1|
    i.+                 # sum
;
```

## Bounded Power

```seq
: bounded-power ( Int Int Int Int -- Int )
    # Stack: base exp min max
    rot rot             # Stack: base min max exp
    rot                 # Stack: base min exp max
    >r >r               # Stash min, max
    imath.pow           # Calculate power
    r> r> imath.clamp   # Clamp result
;
```
