# Hint: String to Integer

The `string->int` function parses a string as an integer.
It returns `(Int Bool)` - the value and a success flag.

## Solution

```seq
"42" string->int drop 2 i.*
```

1. `"42"` pushes the string
2. `string->int` parses it, returning `( 42 true )`
3. `drop` removes the success Bool (we know "42" is valid)
4. `2 i.*` multiplies by 2

## Note on Parsing

If the string doesn't represent a valid integer:
```seq
"abc" string->int  # ( 0 false )
```

For robust code, check the Bool before using the value:
```seq
string->int [
  # success - use the value
] [
  drop 0  # failure - drop invalid value, use default
] if
```
