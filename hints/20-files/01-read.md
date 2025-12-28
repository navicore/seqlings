# Hint: Reading Files

## file.read

The `file.read` operation takes a path and returns the file contents as a string.

## Solution

```seq
: describe-read ( -- String )
    "string"
;
```

File contents are always returned as a string. You can then parse or process them as needed.
