# Hint: Processing File Lines

## file.for-each-line+

The quotation receives two values for each line: the line content and a status flag.

```seq
"file.txt" [
    # Stack has: ( line status )
    [   # status was true
        io.write-line true   # Process line, continue
    ] [
        drop false           # EOF, stop
    ] if
] file.for-each-line+
```

## Solution

```seq
: line-processor-takes ( -- String )
    "line-and-status"
;
```
