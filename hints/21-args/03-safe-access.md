# Hint: Safe Argument Access

## Solution

```seq
: get-or-default ( Int String -- String )
    args.get-or
;
```

Just call `args.get-or` directly.
