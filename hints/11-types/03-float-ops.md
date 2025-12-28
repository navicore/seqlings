# Hint: Float Operations

## Solution

```seq
: test-float-multiply ( -- )
    2.5 4.0 f.multiply
    10.0 f.= test.assert
;

: test-float-divide ( -- )
    7.5 2.5 f.divide
    3.0 f.= test.assert
;
```

Float operations: `f.add`, `f.subtract`, `f.multiply`, `f.divide`

Use `f.=` to compare floats (handles precision issues).
