# Hint: Combining Operations

Operations chain naturally - each one transforms the stack for the next.

## Tracing the Stack

Let's compute (10 + 5) * 2:

```seq
10         # Stack: ( 10 )
5          # Stack: ( 10 5 )
i.add      # Stack: ( 15 )
2          # Stack: ( 15 2 )
i.multiply # Stack: ( 30 )
```

## The Power of Composition

This is **function composition** in action. Each operation is a function that takes the stack and returns a new stack. Chaining them creates a pipeline:

```
stack → add → multiply → result
```

This pattern is universal in programming: Unix pipes, method chains, functional composition.

## Solution

```seq
10 5 i.add 2 i.multiply
```
