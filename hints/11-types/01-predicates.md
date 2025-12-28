# Hint: Understanding Types

## Solution

Each type has its own operations:

```seq
: add-floats ( Float Float -- Float )
    f.add
;

: concat-strings ( String String -- String )
    string.concat
;
```

Use `i.add` for integers, `f.add` for floats, `string.concat` for strings.
