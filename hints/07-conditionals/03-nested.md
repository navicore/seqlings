# Hint: Nested Conditionals

You can put if/else inside another if/else.

## Solution

```seq
15
dup 10 i.> if
    drop 3
else
    dup 5 i.> if
        drop 2
    else
        drop 1
    then
then
```

## Tips for Deeply Nested Conditionals

When nesting gets too deep, consider extracting helper words to simplify the logic.
