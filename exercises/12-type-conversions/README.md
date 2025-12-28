# Type Conversions

Seq is dynamically typed but strongly typed - values have types at runtime, and operations expect specific types. When you need to work across type boundaries, you use explicit conversion functions.

## Available Conversions

### Numeric Conversions
- `int->float` - Convert an integer to a floating-point number
- `float->int` - Convert a float to an integer (truncates toward zero)

### String Conversions
- `int->string` - Convert an integer to its string representation
- `float->string` - Convert a float to its string representation
- `string->int` - Parse a string as an integer (may fail)
- `string->float` - Parse a string as a float (may fail)

## Examples

```seq
42 int->float      # Stack: ( 42.0 )
3.7 float->int     # Stack: ( 3 )
123 int->string    # Stack: ( "123" )
"456" string->int  # Stack: ( 456 )
```

## Why Explicit Conversions?

Implicit type coercion (like JavaScript's `"5" + 3 = "53"`) can lead to subtle bugs. Seq requires you to be explicit about conversions, making code behavior predictable and self-documenting.

## Stack Effects

All conversion functions follow a simple pattern:
```
type->other ( type -- other )
```

They pop one value and push the converted result.

## Exercises in This Section

1. **int-to-string** - Convert integers to their string representation
2. **string-to-int** - Parse strings as integers
3. **int-float** - Convert between integers and floats
4. **float-string** - Convert floats to/from strings
5. **combine** - Chain multiple conversions together
