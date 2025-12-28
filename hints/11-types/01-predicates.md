# Hint: Understanding Types

## Solution

Each type has its own operations:

```seq
: add-floats ( Float Float -- Float )
    f.+
;

: concat-strings ( String String -- String )
    string.concat
;
```

Use `i.+` for integers, `f.+` for floats, `string.concat` for strings.
