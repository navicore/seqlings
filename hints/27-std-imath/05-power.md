# Hint: Integer Power

Push 2 as the base, swap so the exponent is on top, then call
`pow`.

## Solution

```seq
: power-of-2 ( Int -- Int )
    2 swap pow
;
```
