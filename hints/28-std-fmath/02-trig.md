# Hint: Trigonometric Functions

## Solution

```seq
: test-cos-zero ( -- )
    0.0 fmath.cos
    1.0 f.= test.assert
;

: test-sin-pi-half ( -- )
    fmath.pi 2.0 f.divide fmath.sin
    1.0 f.= test.assert
;
```

Use `fmath.pi` for the pi constant.
