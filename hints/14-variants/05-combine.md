# Hint: Building with Variants

## Solution

```seq
: safe-divide ( Int Int -- Result )
    dup 0 =
    [ drop drop "division by zero" Err ]
    [ i.divide Ok ]
    if-else
;
```

Check if divisor is zero first, then either return error or compute result.
