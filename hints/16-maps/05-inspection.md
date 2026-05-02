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
    "tmp" 0 map.set "tmp" map.remove
    map.empty?
    test.assert
;

: test-keys-count ( -- )
    make-test-map
    map.keys list.length
    3 test.assert-eq
;
```

- `map.size` returns the number of key-value pairs
- `map.empty?` returns `true` if the map has no entries — assert with plain `test.assert` (which expects a Bool), not `test.assert-eq` (which is integers-only)
- `map.keys` returns a list of all keys
- The `"tmp" 0 map.set "tmp" map.remove` dance in `test-empty` is just to bind the map's value type to `Int` so the typechecker is happy; the map ends up empty either way
