# Hint: Integer Power

Push 2 as the base, swap so the exponent is on top, call `i.pow`,
then drop its success Bool (since `2^n` for non-negative `n` never
overflows below 64 bits for the test's inputs).

## Solution

```seq
: power-of-2 ( Int -- Int )
    2 swap i.pow drop
;
```
