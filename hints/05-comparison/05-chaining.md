# Hint: Chaining Comparisons

To check multiple conditions, you need to preserve values and combine results.

## The Challenge

We want to check: 5 > 3 AND 5 < 10

But each comparison consumes its operands! So we need to:
1. Keep a copy of 5 for the second comparison
2. Combine the two boolean results

## Solution

```seq
5 dup 3 > swap 10 < and
```

Let's trace it:
```
5             Stack: ( 5 )
dup           Stack: ( 5 5 )
3 >           Stack: ( 5 true )    # 5 > 3
swap          Stack: ( true 5 )
10 <          Stack: ( true true ) # 5 < 10
and           Stack: ( true )      # Both true
```

## Why This Matters

This pattern - duplicate before use, then combine results - is fundamental to stack-based programming. You'll use it constantly.
