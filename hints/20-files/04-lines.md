# Hint: Working with Lines

## Splitting on Newlines

Use `string.split` with the newline character:

```seq
"a\nb\nc" "\n" string.split    # Returns list: [ "a" "b" "c" ]
```

## Solution

```seq
: split-on-newlines ( String -- List )
    "\n" string.split
;
```
