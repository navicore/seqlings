# Hint: Finding Matches

`regex.find` returns `( match Bool )` for the first hit. `regex.find-all` returns `( list Bool )` for every hit. When you only need the list, `drop` the Bool.

## Solution

```seq
: find-first-number ( String -- String Bool )
    "\\d+" regex.find
;

: find-all-words ( String -- Variant )
    "\\w+" regex.find-all drop
;
```

## Patterns

- `\d+` — one or more digits (matches the whole number, not just one digit)
- `\w+` — one or more word characters (letters, digits, underscore)
