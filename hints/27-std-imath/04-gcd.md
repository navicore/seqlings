# Hint: Greatest Common Divisor

Compute the gcd of num and denom, divide num by it, then drop
the success flag that `i./` leaves on top.

## Solution

```seq
: simplify-numerator ( Int Int -- Int )
    over over gcd       # ( num denom gcd )
    swap drop           # ( num gcd )
    i./                 # ( quot success )
    drop                # ( quot )
;
```
