# Hint: Float Operations

## Solution

```seq
: test-float-multiply ( -- )
    2.5 4.0 f.*
    10.0 f.= test.assert
;

: test-float-divide ( -- )
    7.5 2.5 f./
    3.0 f.= test.assert
;
```

We use `f.=` to compare floats, which handles floating-point precision.
