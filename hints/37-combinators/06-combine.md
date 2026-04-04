# Hint: Combining Combinators

Compute the product first, then use `dip` to compute the sum underneath:
```seq
: sum-and-product ( Int Int -- Int Int )
    2dup i.* [ i.+ ] dip
;
```

Trace with `( 3 7 )`:
1. `2dup` → `( 3 7 3 7 )`
2. `i.*` → `( 3 7 21 )` — product computed
3. `[ i.+ ] dip` → hides 21, runs `i.+` on `( 3 7 )` → `( 10 )`, restores 21 → `( 10 21 )`

The `dip` lets us "reach under" the product to compute the sum from the original values.
