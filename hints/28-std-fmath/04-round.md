# Hint: Rounding Functions

## Solution

```seq
: test-ceil ( -- )
    3.2 fmath.ceil
    4.0 f.= test.assert
;

: test-round ( -- )
    2.6 fmath.round
    3.0 f.= test.assert
;

: test-abs ( -- )
    -7.5 fmath.abs
    7.5 f.= test.assert
;
```
