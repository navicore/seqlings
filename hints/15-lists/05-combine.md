# Hint: Combining Operations

Chain filter then fold. `i.modulo` returns `( remainder success )`,
so `drop` the flag before the equality check.

## Solution

```seq
: product-of-odds ( Variant -- Int )
    [ 2 i.modulo drop 1 i.= ] list.filter
    1 [ i.* ] list.fold
;
```

## The Pipeline

```
[ 1 2 3 4 5 ]
  → filter odds → [ 1 3 5 ]
  → fold multiply → 1 * 3 * 5 = 15
```

Each operation flows into the next. This compositional style is a hallmark of functional programming.
