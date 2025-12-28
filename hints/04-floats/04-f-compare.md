# Hint: Float Comparisons

Float comparisons produce boolean results (true/false).

## The Comparison Operators

```seq
5.5 3.0 f.>     # true (5.5 is greater than 3.0)
3.0 5.5 f.<     # true (3.0 is less than 5.5)
2.0 2.0 f.=     # true (equal)
2.0 2.0 f.<>    # false (not not-equal)
```

## Solution

```seq
5.5 3.0 f.>
```

## Caution: Float Equality

Be careful with `f.=` on computed values:

```seq
0.1 0.2 f.+ 0.3 f.=    # Might be false!
```

Due to floating-point representation, `0.1 + 0.2` may not equal exactly `0.3`. For computed values, compare within a tolerance instead of exact equality. This is a fundamental issue in all programming languages.
