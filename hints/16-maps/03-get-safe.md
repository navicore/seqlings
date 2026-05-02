# Hint: Safe Access

There is no `map.get-or` builtin. `map.get` already returns a found-Bool — branch on it with `if`.

## Pattern

```seq
my-map "key" map.get      # ( Value Bool )
[ ... use Value ... ]
[ drop ... fallback ... ]
if
```

## Solution for `get-or`

The arguments arrive as `( Map Key Default )`, but `map.get` wants `( Map Key )` on top. Use `rot rot` to bury Default below Map and Key:

```seq
: get-or ( Map String String -- String )
    rot rot map.get
    [ nip ]                # found: drop Default below the value
    [ drop ]               # missing: drop the placeholder value
    if
;
```

Trace for `( map "name" "Unknown" )`:

- `rot rot` → `( "Unknown" map "name" )`
- `map.get` → `( "Unknown" "Bob" true )`
- found-quotation `[ nip ]` → `( "Bob" )`

For `( map "age" "Unknown" )`:

- `rot rot` → `( "Unknown" map "age" )`
- `map.get` → `( "Unknown" 0 false )` (placeholder value, not-found)
- missing-quotation `[ drop ]` → `( "Unknown" )`
