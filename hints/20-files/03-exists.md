# Hint: File Existence

## file.exists?

Returns an Int (0 or 1) indicating whether the file exists.

## Solution

```seq
: exists-returns ( -- String )
    "int"
;
```

Use this before file.slurp to check if the file exists first.
