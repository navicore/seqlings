# Hint: Float Literals

A float literal must contain a decimal point.

## The Difference

```seq
42      # Integer
42.0    # Float
```

Even though both represent "forty-two," they are different types in Seq.

## Solution

Simply push the literal:

```seq
2.5
```

## Why Types Matter

Distinguishing int from float prevents subtle bugs:
- Integer overflow vs float precision loss are different problems
- `5 / 2` vs `5.0 / 2.0` have different results
- Some algorithms require exact integers (counting, indexing)
- Others require continuous values (physics, graphics)

Explicit types make you think about which you need.
