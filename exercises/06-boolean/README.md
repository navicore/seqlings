# Part 6: Boolean Logic

Boolean algebra is the mathematics of truth values. Every conditional, every loop, every decision in every program ultimately reduces to boolean operations.

## The Boolean Values

Seq has two boolean values:
- `true`
- `false`

## Boolean Operators

| Word  | Effect                    | Description           |
|-------|---------------------------|-----------------------|
| `and` | `( a b -- bool )`         | True if both true     |
| `or`  | `( a b -- bool )`         | True if either true   |
| `not` | `( bool -- bool )`        | Inverts the value     |

## Truth Tables

**and**:
```
true  true  and  → true
true  false and  → false
false true  and  → false
false false and  → false
```

**or**:
```
true  true  or   → true
true  false or   → true
false true  or   → true
false false or   → false
```

**not**:
```
true  not  → false
false not  → true
```

## Why Boolean Logic Matters

Boolean algebra was invented by George Boole in 1847, long before computers. It's the mathematical foundation that makes digital computers possible.

Every AND gate, OR gate, and NOT gate in your CPU implements these operations in hardware. When you write `and`, `or`, `not`, you're directly expressing the logic gates that execute your code.

## De Morgan's Laws

These equivalences are fundamental:

```
not (a and b)  ≡  (not a) or (not b)
not (a or b)   ≡  (not a) and (not b)
```

Understanding these helps you simplify complex conditions.
