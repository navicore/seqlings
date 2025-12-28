# Part 5: Comparison Operators

Comparisons are the foundation of decision-making in programs. Every `if`, every loop condition, every filter ultimately rests on comparison.

## Boolean Results

Comparison operators produce boolean values:
- `true` - the comparison holds
- `false` - the comparison doesn't hold

## Integer Comparison Operators

Like arithmetic, comparison operators are prefixed with `i.` for integers:

| Symbol  | Word Form    | Effect              | Meaning                |
|---------|--------------|---------------------|------------------------|
| `i.=`   | `i.eq`       | `( a b -- bool )`   | Equal                  |
| `i.<>`  | `i.neq`      | `( a b -- bool )`   | Not equal              |
| `i.<`   | `i.lt`       | `( a b -- bool )`   | Less than              |
| `i.>`   | `i.gt`       | `( a b -- bool )`   | Greater than           |
| `i.<=`  | `i.lte`      | `( a b -- bool )`   | Less than or equal     |
| `i.>=`  | `i.gte`      | `( a b -- bool )`   | Greater than or equal  |

Both forms are equivalent - use whichever reads better in context. Symbol forms are more common in practice.

## Operand Order

For non-symmetric comparisons, order matters:

```seq
3 5 i.<    # true  (3 < 5)
5 3 i.<    # false (5 < 3)
```

Think: "first value, compared to second value."

## Comparison as Transformation

A comparison transforms two values into one truth value:

```seq
( a b ) → ( bool )
```

This is **abstraction** - hiding the details of the numbers and exposing only the relationship between them. The boolean result is information about the comparison, not about either operand.

## Chaining Comparisons

Multiple comparisons can be chained:

```seq
10 5 i.>    # true
3 i.=       # Compare 10>5 result (true) with 3? Type error!
```

Wait - that's a type error! Booleans and integers are different types. You can't compare `true` with `3`. To check multiple conditions, you need boolean logic (next section).
