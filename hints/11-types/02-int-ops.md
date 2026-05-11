# Hint: Integer Operations

Both `i./` and `i.modulo` return `( Int Bool )` — the quotient
(or remainder) plus a success flag that's `false` only when the
divisor is zero. Assert the flag first, then assert the value.

## Solution

```seq
: test-int-divide ( -- )
    17 5 i./
    test.assert
    3 test.assert-eq
;

: test-int-mod ( -- )
    17 5 i.modulo
    test.assert
    2 test.assert-eq
;
```

Integer operations: `i.+`, `i.-`, `i.*`, `i./`, `i.modulo`.
