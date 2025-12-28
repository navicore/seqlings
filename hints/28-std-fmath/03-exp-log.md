# Hint: Exponential and Logarithmic

## Solution

```seq
: test-log10 ( -- )
    100.0 fmath.log10
    2.0 f.= test.assert
;

: test-pow ( -- )
    2.0 8.0 fmath.pow
    256.0 f.= test.assert
;
```

`fmath.pow` takes base and exponent.
