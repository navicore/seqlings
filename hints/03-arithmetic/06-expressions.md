# Hint: Complex Expressions

For expressions with multiple sub-expressions, compute each part, then combine.

## The Expression

Infix: (20 - 5) * (8 + 2)

## Strategy

1. Compute (20 - 5) = 15
2. Compute (8 + 2) = 10
3. Multiply the results: 15 * 10 = 150

## Stack Trace

```seq
20 5 i.-    # Stack: ( 15 )
8 2 i.+          # Stack: ( 15 10 )
i.*         # Stack: ( 150 )
```

After step 1, the 15 stays on the stack. After step 2, both 15 and 10 are there. The final multiply consumes both.

## Why No Parentheses?

In postfix notation, the structure is implicit in the order. No parentheses needed, no precedence rules to remember. The code IS the order of operations.

## Solution

```seq
20 5 i.- 8 2 i.+ i.*
```
