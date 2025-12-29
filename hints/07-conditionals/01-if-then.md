# Hint: Simple if/then

The `if` keyword consumes a boolean and conditionally executes code.

**Key insight:** The if body must have a balanced stack effect - it can't just push a value, because when the condition is false, the body is skipped and the stack would be different.

## Pattern

Push a default value first, then conditionally replace it:

```seq
0              # default
10 5 i.> if
    drop 100   # replace default with 100
then
```

## How It Works

1. `0` pushes a default value
2. `10 5 i.>` pushes `true`
3. `if` pops the boolean and evaluates the body (since true)
4. `drop 100` removes the 0 and pushes 100
5. `then` marks the end of the conditional

If the condition were false, the body would be skipped and you'd have the default `0` on the stack.
