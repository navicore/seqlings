# Hint: i./

`i./` performs **integer division** (truncates toward zero).

## Understanding Integer Division

```seq
20 4 i./    # 20 / 4 = 5 exactly
17 5 i./    # 17 / 5 = 3 (remainder 2 is discarded)
100 8 i./   # 100 / 8 = 12 (remainder 4 is discarded)
```

Like subtraction, order matters: first operand divided BY second operand.

## Solution

```seq
100 8 i./
```

## Note on Remainders

To get the remainder, use `i.mod` (covered later). Integer division and modulo together give you the quotient and remainder.
