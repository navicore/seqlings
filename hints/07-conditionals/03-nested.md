# Hint: Nested Conditionals

You can put if/else inside another if/else.

## Solution

```seq
15
dup 10 > if
    drop 3
else
    dup 5 > if
        drop 2
    else
        drop 1
    then
then
```

## Alternative with cond

Nested ifs get unwieldy. For multiple conditions, `cond` is cleaner (next exercise).
