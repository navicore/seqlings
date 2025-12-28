# Part 5: Comparison Operators

Comparisons are the foundation of decision-making in programs. Every `if`, every loop condition, every filter ultimately rests on comparison.

## Boolean Results

Comparison operators produce boolean values:
- `true` - the comparison holds
- `false` - the comparison doesn't hold

## Integer Comparison Operators

| Word  | Effect              | Meaning                |
|-------|---------------------|------------------------|
| `=`   | `( a b -- bool )`   | Equal                  |
| `<>`  | `( a b -- bool )`   | Not equal              |
| `<`   | `( a b -- bool )`   | Less than              |
| `>`   | `( a b -- bool )`   | Greater than           |
| `<=`  | `( a b -- bool )`   | Less than or equal     |
| `>=`  | `( a b -- bool )`   | Greater than or equal  |

## Operand Order

For non-symmetric comparisons, order matters:

```seq
3 5 <    # true  (3 < 5)
5 3 <    # false (5 < 3)
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
10 5 >    # true
3 =       # Compare 10>5 result (true) with 3? Type error!
```

Wait - that's a type error! Booleans and integers are different types. You can't compare `true` with `3`. To check multiple conditions, you need boolean logic (next section).
