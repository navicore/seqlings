# Hint: Integer/Float Conversions

## int->float

Converts an integer to a floating-point number:
```seq
7 int->float    # Stack: ( 7.0 )
```

## float->int

Converts a float to an integer by truncating toward zero:
```seq
7.9 float->int    # Stack: ( 7 )
-2.9 float->int   # Stack: ( -2 )
```

## Solution

```seq
7 int->float 0.5 f.add float->int
```

1. `7 int->float` gives 7.0
2. `0.5 f.add` gives 7.5
3. `float->int` truncates to 7

## Key Insight

Truncation always moves toward zero, not toward negative infinity. This matters for negative numbers.
