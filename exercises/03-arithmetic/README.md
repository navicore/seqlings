# Part 3: Arithmetic

Arithmetic in a stack-based language reveals something profound: all computation is about transforming data. In Seq, every operation explicitly shows its inputs and outputs.

## Postfix Notation

Traditional math uses **infix** notation: `3 + 4`. The operator sits between operands.

Seq uses **postfix** notation: `3 4 i.add`. The operator comes after the operands.

This isn't arbitrary - it's deeply practical:
- No operator precedence rules needed
- No parentheses required
- Operations compose naturally left-to-right

## Operand Order Matters

For non-commutative operations (subtraction, division), order matters:

```seq
10 3 i.subtract    # 10 - 3 = 7   (NOT 3 - 10)
```

The first value pushed is the left operand. Think: "10, then 3, now subtract."

## The Integer Operations

| Word         | Effect                    | Description              |
|--------------|---------------------------|--------------------------|
| `i.add`      | `( a b -- sum )`          | a + b                    |
| `i.subtract` | `( a b -- diff )`         | a - b                    |
| `i.multiply` | `( a b -- product )`      | a × b                    |
| `i.divide`   | `( a b -- quotient )`     | a ÷ b (integer division) |

## Expression Composition

Complex expressions build up naturally:

```seq
# Calculate (3 + 4) * 2
3 4 i.add 2 i.multiply    # Stack: ( 14 )

# Calculate 3 + (4 * 2)
3  4 2 i.multiply  i.add  # Stack: ( 11 )
```

No parentheses in the code - the order of operations is explicit in the sequence.

## Why This Matters

Understanding postfix notation helps you see that expressions are really **sequential transformations**. This insight carries over to:
- Pipeline operators in modern languages (`|>`, `->`, etc.)
- Method chaining (`obj.method1().method2()`)
- Functional composition (`f ∘ g ∘ h`)
