# Hint: File Existence

## file.exists?

Returns a boolean indicating whether the file exists.

## Solution

```seq
: exists-returns ( -- String )
    "bool"
;
```

Use this before file.read to avoid errors on missing files.
