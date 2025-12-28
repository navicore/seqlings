# Hint: String to Integer

The `string->int` function parses a string as an integer.

## Solution

```seq
"42" string->int 2 i.multiply
```

1. `"42"` pushes the string
2. `string->int` parses it to integer 42
3. `2 i.multiply` multiplies by 2

## Note on Parsing

If the string doesn't represent a valid integer, parsing will fail. Always ensure your input strings are valid numbers.
