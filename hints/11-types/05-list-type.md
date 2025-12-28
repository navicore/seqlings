# Hint: List Operations

## Solution

```seq
: test-list-first ( -- )
    [ 5 10 15 ] 0 variant.field-at
    5 test.assert-eq
;

: test-list-last ( -- )
    [ 5 10 15 ] 2 variant.field-at
    15 test.assert-eq
;
```

Lists are variants - access elements with `variant.field-at` using index.
Index 0 is first, index (length-1) is last.
