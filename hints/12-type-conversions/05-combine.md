# Hint: Chaining Conversions

## Solution

```seq
: string-to-quarter ( String -- Int )
    string->int int->float 4.0 f.divide float->int
;
```

Step by step:
1. `string->int` - "100" becomes 100
2. `int->float` - 100 becomes 100.0
3. `4.0 f.divide` - Divide by 4.0 to get 25.0
4. `float->int` - Convert back to integer 25

## Round-Trip Conversions

Integer to string and back preserves the value exactly:
```seq
42 int->string string->int    # Still 42
```

Float round-trips may lose precision due to string formatting.
