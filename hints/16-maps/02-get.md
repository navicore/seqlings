# Hint: Getting Values

`map.get` has stack effect `( Map Key -- Value Bool )` — it returns the value AND a Bool indicating whether the key was found. When you know the key exists, drop the Bool.

## Solution

```seq
: test-get ( -- )
    make-test-map
    "color" map.get drop
    "blue" string.equal? test.assert
;

: test-get-other ( -- )
    make-test-map
    "size" map.get drop
    "large" string.equal? test.assert
;
```

The "do not edit" line uses `string.equal? test.assert` because `test.assert-eq` is integers-only — for strings, compute the comparison Bool with `string.equal?` and assert it.
