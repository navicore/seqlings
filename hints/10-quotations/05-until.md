# Hint: The until Loop

`until` is like `while`, but runs UNTIL the condition becomes true.

## Solution

```seq
3
[ dup 100 > ] [ 2 i.multiply ] until
```

Trace:
- 3 > 100? No, multiply: 6
- 6 > 100? No, multiply: 12
- 12 > 100? No, multiply: 24
- 24 > 100? No, multiply: 48
- 48 > 100? No, multiply: 96
- 96 > 100? No, multiply: 192
- 192 > 100? Yes, stop

## while vs until

They're duals:
- `while [ condition ]` = run while condition is true
- `until [ condition ]` = run while condition is false

You could implement one in terms of the other:
```seq
[ condition not ] body while   # equivalent to: [ condition ] body until
```
