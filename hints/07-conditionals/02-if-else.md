# Hint: if/else/then

The `else` clause provides code for the false case.

## Solution

```seq
3 7 > if
    1
else
    2
then
```

Since 3 > 7 is false, the else branch runs, pushing 2.

## Both Branches Must Match

Critical rule: both branches must have the same stack effect.

```seq
# GOOD - both branches push one value
condition if 1 else 2 then

# BAD - one pushes 1, other pushes 2 values
condition if 1 else 2 3 then   # Type error!
```
