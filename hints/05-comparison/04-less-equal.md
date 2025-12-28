# Hint: Less-or-Equal (<=)

`<=` and `>=` include the equality case.

## Solution

```seq
7 7 <=
```

7 is less than or equal to 7 (specifically, equal), so the result is `true`.

## Boundary Conditions

`<=` and `>=` are essential for boundary checking:

```seq
index 0 >=           # Is index non-negative?
index length <       # Is index within bounds?
and                  # Both conditions must hold
```

These "fence post" conditions are where many bugs hide.
