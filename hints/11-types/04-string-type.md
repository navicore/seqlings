# Hint: String Type

## Solution

```seq
: test-string-length ( -- )
    "hello" string.length
    5 test.assert-eq
;

: test-string-concat ( -- )
    "hello" " world" string.concat
    "hello world" test.assert-eq
;
```

`string.concat` joins two strings together.
