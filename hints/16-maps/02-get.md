# Hint: Getting Values

## Solution

```seq
: test-get ( -- )
    make-test-map
    "color" map.get
    "blue" test.assert-eq
;

: test-get-other ( -- )
    make-test-map
    "size" map.get
    "large" test.assert-eq
;
```

The `map.get` operation pops the map and key, then pushes the associated value.
