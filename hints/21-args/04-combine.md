# Hint: Building Argument Parsers

## Solution

```seq
: require-args ( Int -- Bool )
    args.count swap >=
;
```

Check if count >= required.
