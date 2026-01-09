# Hint: Clamp

## The Elegant Way: Factor Into Helper Words

When stack manipulation gets complex, **create helper words**. This is the core wisdom of stack-based programming.

### Step 1: Create `min`
```seq
: min ( Int Int -- Int )
    2dup i.> if swap then drop
;
```
If a > b, swap them, then drop the top (larger) value.

### Step 2: Create `max`
```seq
: max ( Int Int -- Int )
    2dup i.< if swap then drop
;
```
Same pattern, opposite comparison.

### Step 3: Now `clamp` is beautiful!
```seq
: clamp ( Int Int Int -- Int )
    # Stack: ( value min max )
    rot swap    # ( max value min )
    min         # ( max clamped-to-min )
    max         # ( result )
;
```

The logic: take the min of (value, max), then take the max of (that, min).

This reads almost like English and is trivially correct. The "complex" stack problem vanishes when you factor it properly.
