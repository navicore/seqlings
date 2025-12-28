# Hint: Simple if/then

The `if` keyword consumes a boolean and conditionally executes code.

## Solution

```seq
10 5 > if
    100
then
```

Since 10 > 5 is true, 100 gets pushed.

## How It Works

1. `10 5 >` pushes `true`
2. `if` pops the `true` and evaluates the body
3. `100` gets pushed
4. `then` marks the end of the conditional
