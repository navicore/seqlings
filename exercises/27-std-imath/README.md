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

### Sign
```seq
42 sign     # Stack: ( 1 )
-7 sign     # Stack: ( -1 )
 0 sign     # Stack: ( 0 )
```

### Power (builtin, not in std:imath)

Integer exponentiation is the builtin `i.pow ( Int Int -- Int Bool )` —
it returns a success Bool alongside the result (false on negative
exponent, overflow, or exponent > u32::MAX):

```seq
2 10 i.pow drop    # Stack: ( 1024 ) — 2^10
```

`i.pow` lives in the builtins, not the std:imath module — so unlike
the other words on this page, it stays prefixed even after `include
std:imath`. Bare-name sugar exists for the other integer ops
(`+`, `-`, `*`, `/`, `<`, `>`, `=`, `<>`) but not yet for `pow`
(patch-seq #489).

## Why Use std:imath?

The builtins (`i.+`, `i.-`, `i.*`, `i./`, `i.<`, etc., most accessible bare via sugar) give you the *primitives*. `std:imath` provides the higher-level helpers you'd otherwise reinvent every time: `abs`, `min`, `max`, `gcd`, `clamp`, `sign`, `square`. None of those are builtins — they're plain Seq words implemented on top of the arithmetic primitives, and the chapter walks through using them.

`i.pow` sits in the middle: it's a builtin (not a std:imath helper), but the operations are conceptually adjacent enough that exercise 5 brings it in.

## Exercises in This Section

1. **abs** — absolute value
2. **min-max** — finding minimum and maximum
3. **clamp** — constraining values to ranges
4. **gcd** — greatest common divisor
5. **power** — integer exponentiation
6. **combine** — combining math operations
