# Hint: i.add

`i.add` takes two integers from the stack and pushes their sum.

## The Pattern

```seq
a b i.add    # Leaves (a + b) on stack
```

## Solution

Push 10, push 25, then add:

```seq
10 25 i.add
```

## Thinking in Transformations

Every operation transforms the stack. `i.add` transforms `( a b )` into `( sum )`. This transformation mindset is key to functional programming - data flows through a series of transformations.
