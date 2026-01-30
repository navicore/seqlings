# Hint: Building with Variants

## Check before dividing

Check if divisor is zero BEFORE calling `i./` to avoid the error case:

```seq
: safe-divide ( Int Int -- IntResult )
    dup 0 i.= if
        drop drop
        "division by zero" Make-Err
    else
        i./ drop    # drop the success Bool, we checked already
        Make-Ok
    then
;
```

## Why check first?

`i./` returns `(quotient success)`. By checking for zero first, we:
1. Avoid the division entirely when it would fail
2. Can safely drop the success Bool since we know it's true
