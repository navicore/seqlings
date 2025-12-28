# Hint: i.multiply

`i.multiply` takes two integers and pushes their product.

## The Pattern

```seq
a b i.multiply    # Leaves (a × b) on stack
```

Since multiplication is commutative (a × b = b × a), order doesn't matter for the result.

## Solution

```seq
12 5 i.multiply
```

Or equivalently:

```seq
5 12 i.multiply
```
