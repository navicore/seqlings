# Hint: String Operations

## Solution

```seq
: test-string-concat ( -- )
    "hello" " world" string.concat
    "hello world" string.equal? test.assert
;
```

Use `string.concat` to join strings, `string.equal?` to compare them.
