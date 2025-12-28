# Hint: Combining Operations

Chain filter then fold.

## Solution

```seq
: product-of-odds ( List -- Int )
    [ 2 i.mod 1 = ] list.filter
    1 [ i.multiply ] list.fold
;
```

## The Pipeline

```
[ 1 2 3 4 5 ]
  → filter odds → [ 1 3 5 ]
  → fold multiply → 1 * 3 * 5 = 15
```

Each operation flows into the next. This compositional style is a hallmark of functional programming.
