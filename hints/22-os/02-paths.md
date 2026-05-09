# Hint: Path Operations

Three branches, two predicates. `dup` the path before each
predicate so the path is still on the stack if the predicate
returns false and you need to ask a different question.

## Solution

```seq
: path-status ( path -- String )
    dup os.path-is-dir
    [ drop "dir" ]
    [ dup os.path-is-file
        [ drop "file" ]
        [ drop "missing" ]
        if
    ]
    if
;
```
