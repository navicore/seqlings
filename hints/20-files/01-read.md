# Hint: Reading Files

## file.slurp

The `file.slurp` operation takes a path and returns the file contents as a string.

## Solution

```seq
: describe-slurp ( -- String )
    "string"
;
```

File contents are always returned as a string. You can then parse or process them as needed.
