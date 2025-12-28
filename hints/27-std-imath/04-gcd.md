# Hint: Greatest Common Divisor

## Solution

```seq
: simplify-numerator ( Int Int -- Int )
    # Stack: num denom
    2dup imath.gcd    # Stack: num denom gcd
    rot swap i./      # Stack: denom (num/gcd)
    swap drop         # Stack: (num/gcd)
;
```

Or more directly:

```seq
: simplify-numerator ( Int Int -- Int )
    over over imath.gcd  # num denom gcd
    swap drop            # num gcd
    i./                  # num/gcd
;
```
