# Hint: File Reading Patterns

## Safe vs Unsafe Reading

`file.slurp` panics if the file doesn't exist.
`file.slurp-safe` returns a status flag instead.

For robust code, use `file.slurp-safe`:

```seq
"config.txt" file.slurp-safe
if
    process-config
else
    drop use-defaults
then
```

## Solution

```seq
: safest-approach ( -- String )
    "slurp-safe"
;
```
