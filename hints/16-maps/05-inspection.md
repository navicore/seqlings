# Hint: Map Inspection

## Solution

```seq
: test-size ( -- )
    make-test-map
    map.size
    3 test.assert-eq
;

: test-empty ( -- )
    map.make
    map.empty?
    true test.assert-eq
;

: test-keys-count ( -- )
    make-test-map
    map.keys list.length
    3 test.assert-eq
;
```

- `map.size` returns the number of key-value pairs
- `map.empty?` returns true if the map has no entries
- `map.keys` returns a list of all keys
