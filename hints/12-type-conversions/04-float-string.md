# Hint: Float/String Conversions

## string->float

Parse a string as a floating-point number:
```seq
"3.5" string->float    # Stack: ( 3.5 )
```

## Solution

```seq
"3.5" string->float 1.5 f.+
```

1. `"3.5" string->float` gives 3.5
2. `1.5 f.+` adds to get 5.0

## float->string

Convert a float to its string representation:
```seq
3.14159 float->string    # Stack: ( "3.14159" )
```
