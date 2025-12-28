# Hint: f.* and f./

Float division gives you the true quotient, not a truncated integer.

## The Key Difference

```seq
7 2 i./      # Integer division: 3
7.0 2.0 f./  # Float division: 3.5
```

## Solution

```seq
15.0 4.0 f./
```

Result: 3.75

## Real-World Impact

This difference matters enormously in real programs:
- Financial calculations need decimal precision
- Graphics need smooth interpolation
- Statistics need proper averages

Choosing the right numeric type is a fundamental design decision.
