# Standard Library: Integer Math (std:imath)

The `std:imath` module provides additional integer math operations beyond the basic built-ins. Import it with:

```seq
include std:imath
```

After the include, the imath words are accessible by their bare names — `abs`, `min`, `max`, etc. There's no `imath.` prefix.

## Available Functions

### Absolute Value
```seq
-5 abs    # Stack: ( 5 )
```

### Min and Max
```seq
3 7 min    # Stack: ( 3 )
3 7 max    # Stack: ( 7 )
```

### Clamping
```seq
15 0 10 clamp    # Stack: ( 10 ) — clamps 15 to range [0,10]
-5 0 10 clamp    # Stack: ( 0 )
 5 0 10 clamp    # Stack: ( 5 )
```

### Greatest Common Divisor
```seq
12 8 gcd    # Stack: ( 4 )
```

### Power
```seq
2 10 pow    # Stack: ( 1024 ) — 2^10
```

### Sign
```seq
42 sign     # Stack: ( 1 )
-7 sign     # Stack: ( -1 )
 0 sign     # Stack: ( 0 )
```

## Why Use std:imath?

The built-ins give you `i.+ i.- i.* i./ i.<` etc. — primitive integer arithmetic. `std:imath` builds on those for the operations you keep wanting: absolute value, min/max, gcd, power, clamp.

## Exercises in This Section

1. **abs** — absolute value
2. **min-max** — finding minimum and maximum
3. **clamp** — constraining values to ranges
4. **gcd** — greatest common divisor
5. **power** — integer exponentiation
6. **combine** — combining math operations
