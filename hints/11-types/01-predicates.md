# Hint: Type Predicates

## Solution

```seq
: test-string-predicate ( -- )
    "hello" string?
    true test.assert-eq
;

: test-not-int ( -- )
    "42" int?
    false test.assert-eq
;
```

The string `"42"` is NOT an integer - it's a string that happens to contain digits.
