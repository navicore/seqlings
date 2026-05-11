# Hint: Trigonometric Functions

`f.pi` is the constant. `f.pi 2.0 f./` is π/2.

## Solution

```seq
: test-cos-zero ( -- )
    0.0 f.cos
    1.0 f.= test.assert
;

: test-sin-pi-half ( -- )
    f.pi 2.0 f./ f.sin
    1.0 f.= test.assert
;
```
