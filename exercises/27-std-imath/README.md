# Standard Library: Integer Math (std:imath)

The `std:imath` module provides additional integer math operations beyond the basic built-ins. Import it with:

```seq
use std:imath
```

## Available Functions

### Absolute Value
```seq
-5 imath.abs    # Stack: ( 5 )
```

### Min and Max
```seq
3 7 imath.min    # Stack: ( 3 )
3 7 imath.max    # Stack: ( 7 )
```

### Clamping
```seq
15 0 10 imath.clamp    # Stack: ( 10 ) - clamps 15 to range [0,10]
-5 0 10 imath.clamp    # Stack: ( 0 )
5 0 10 imath.clamp     # Stack: ( 5 )
```

### Greatest Common Divisor
```seq
12 8 imath.gcd    # Stack: ( 4 )
```

### Power
```seq
2 10 imath.pow    # Stack: ( 1024 ) - 2^10
```

### Sign
```seq
42 imath.sign     # Stack: ( 1 )
-7 imath.sign     # Stack: ( -1 )
0 imath.sign      # Stack: ( 0 )
```

## Why Use std:imath?

These functions handle edge cases properly:
- `imath.abs` works correctly with integer bounds
- `imath.gcd` handles negative numbers
- `imath.clamp` ensures values stay in bounds

## Exercises in This Section

1. **abs** - Absolute value
2. **min-max** - Finding minimum and maximum
3. **clamp** - Constraining values to ranges
4. **gcd** - Greatest common divisor
5. **power** - Integer exponentiation
6. **combine** - Combining math operations
