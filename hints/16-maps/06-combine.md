# Hint: Building a Map Workflow

Maps in Seq are homogeneous in their value type. This profile uses Strings for every field (name, email, city), so `make-profile` takes three Strings.

## Solution

```seq
: make-profile ( String String String -- Map )
    # Stack: name email city (city on top)
    map.make
    "city" rot map.set
    "email" rot map.set
    "name" rot map.set
;

: update-city ( Map String -- Map )
    "city" swap map.set
;
```

The `rot` trick: each `"key" rot` pulls the next value (from below the empty/partial map) up so `map.set` can consume `( Map Key Value )`.

Trace for `( "Alice" "alice@example.com" "Portland" )`:

- `map.make` → `( "Alice" "alice@example.com" "Portland" map )`
- `"city" rot` → `( "Alice" "alice@example.com" map "city" "Portland" )`
- `map.set` → `( "Alice" "alice@example.com" map' )` where `map'` has `city: "Portland"`
- repeat for `email` and `name`

The tests use `map.get drop` to discard the found-Bool, then `string.equal? test.assert` to compare strings (since `test.assert-eq` is integers-only).
