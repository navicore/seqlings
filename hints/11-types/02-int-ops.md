# Hint: Integer Operations

## Solution

```seq
: test-int-divide ( -- )
    17 5 i./
    3 test.assert-eq
;

: test-int-mod ( -- )
    17 5 i.mod
    2 test.assert-eq
;
```

Integer division truncates toward zero. Modulo gives the remainder.
