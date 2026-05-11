# Hint: Exponential and Logarithmic

`f.pow` argument order is `base exp` — `2.0 8.0 f.pow` is 2⁸.

## Solution

```seq
: test-log10 ( -- )
    100.0 f.log10
    2.0 f.= test.assert
;

: test-pow ( -- )
    2.0 8.0 f.pow
    256.0 f.= test.assert
;
```
