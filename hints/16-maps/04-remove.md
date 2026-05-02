# Hint: Removing Entries

## map.remove

Removes a key from the map and returns the updated map.

## map.has?

Checks if a key exists and returns a Bool.

## Solution

```seq
: test-remove ( -- )
    make-test-map
    "b" map.remove
    "b" map.has? test.assert-not
;

: test-contains ( -- )
    make-test-map
    "a" map.has?
    test.assert
;
```

`test.assert` asserts that the Bool on the stack is `true`; `test.assert-not` asserts it is `false`. (Note: `test.assert-eq` only works for integers — use `test.assert` / `test.assert-not` for Bool.)
