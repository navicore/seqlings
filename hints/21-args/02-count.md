# Hint: Counting Arguments

## Solution

```seq
: has-args ( Int -- Bool )
    args.count swap >
;
```

Compare the argument count with n.
