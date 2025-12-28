# Hint: Safe File Reading

## file.slurp-safe

`file.slurp-safe` returns two values: the contents and a status flag.

```seq
"file.txt" file.slurp-safe   # ( -- contents status )
```

The status is 1 (true) if reading succeeded, 0 (false) if it failed.

## Solution

```seq
: returns-status? ( -- Int )
    1
;
```
