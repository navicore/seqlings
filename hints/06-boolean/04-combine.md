# Hint: Complex Boolean Expressions

Chain operators to build complex conditions.

## Solution

```seq
true false and true or
```

Trace:
1. `true false and` → `false`
2. `false true or` → `true`

## Postfix Clarity

In postfix, there's no operator precedence confusion. The order of words IS the order of evaluation. Compare:

**Infix (ambiguous without parentheses):**
```
a AND b OR c    # Is this (a AND b) OR c, or a AND (b OR c)?
```

**Postfix (unambiguous):**
```
a b and c or    # Clearly (a AND b) OR c
a b c or and    # Clearly a AND (b OR c)
```
