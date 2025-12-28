# Hint: i.-

`i.-` computes: first operand MINUS second operand.

## The Order

```seq
100 42 i.-    # 100 - 42 = 58
42 100 i.-    # 42 - 100 = -58
```

The value pushed **first** is the minuend (what you subtract from).
The value pushed **second** is the subtrahend (what you subtract).

## Why This Order?

This matches how you'd read it left-to-right: "100, 42, subtract" means "100 minus 42." The stack builds up the expression as you write it.

## Solution

```seq
100 42 i.-
```
