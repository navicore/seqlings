# Hint: Mixing Types

When mixing integers and floats, you must explicitly convert.

## Conversion Words

```seq
42 int->float        # 42.0
3.7 float->int       # 3 (truncates toward zero)
```

## Solution

```seq
5 int->float 2.5 f.*
```

Step by step:
1. Push 5 (an integer)
2. Convert to 5.0 (a float)
3. Push 2.5
4. Multiply: 5.0 × 2.5 = 12.5

## Why Explicit Conversion?

Many bugs come from implicit type conversions:
- Python 2's `/` vs Python 3's `/`
- JavaScript's `"5" + 3` vs `"5" - 3`
- C's integer promotion rules

Seq's explicit conversions prevent surprises. When you see `int->float`, you know exactly what's happening.

## Type System Philosophy

This is your first taste of Seq's type philosophy: **make the programmer's intent clear in the code**. The code documents itself.
