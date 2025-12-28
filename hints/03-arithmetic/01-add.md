# Hint: i.+

`i.+` takes two integers from the stack and pushes their sum.

## The Pattern

```seq
a b i.+    # Leaves (a + b) on stack
```

## Solution

Push 10, push 25, then add:

```seq
10 25 i.+
```

## Thinking in Transformations

Every operation transforms the stack. `i.+` transforms `( a b )` into `( sum )`. This transformation mindset is key to functional programming - data flows through a series of transformations.
