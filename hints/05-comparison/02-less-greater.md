# Hint: Less Than and Greater Than

`<` and `>` compare magnitude.

## Solution

```seq
10 7 >
```

10 is greater than 7, so the result is `true`.

## Order Matters

```seq
10 7 >    # true (10 > 7)
7 10 >    # false (7 > 10)
```

First operand is compared TO second operand.
