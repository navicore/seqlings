# Hint: Removing Entries

## map.remove

Removes a key from the map and returns the updated map.

## map.contains?

Checks if a key exists and returns a boolean.

## Solution

```seq
: test-remove ( -- )
    make-test-map
    "b" map.remove
    "b" map.contains?
    false test.assert-eq
;

: test-contains ( -- )
    make-test-map
    "a" map.contains?
    true test.assert-eq
;
```
