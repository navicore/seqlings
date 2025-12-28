# Hint: Writing Files

## Argument Order

In Seq, the pattern is "data before destination":

```seq
content path file.write
```

So content comes first (deeper on the stack), then path.

## Solution

```seq
: write-order ( -- String )
    "content"
;
```
