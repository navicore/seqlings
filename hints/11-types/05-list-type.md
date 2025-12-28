# Hint: List Type

## Solution

```seq
: test-list-length ( -- )
    [ 10 20 30 40 ] list.length
    4 test.assert-eq
;

: test-list-first ( -- )
    [ 5 10 15 ] list.first
    5 test.assert-eq
;
```

`list.first` gets the first element, `list.length` counts elements.
