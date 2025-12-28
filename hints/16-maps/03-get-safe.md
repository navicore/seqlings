# Hint: Safe Access

## map.get-or

Syntax: `map key default map.get-or`

Returns the value if the key exists, otherwise returns the default.

## Solution

```seq
: test-get-existing ( -- )
    make-test-map
    "name" "Unknown" map.get-or
    "Bob" test.assert-eq
;

: test-get-missing ( -- )
    make-test-map
    "age" "Unknown" map.get-or
    "Unknown" test.assert-eq
;
```

When "name" exists, it returns "Bob". When "age" doesn't exist, it returns "Unknown".
