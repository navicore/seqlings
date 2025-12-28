# Hint: Variant Basics

## Solution

```seq
: test-create-go ( -- )
    Go variant.tag
    "Go" test.assert-eq
;
```

Each variant constructor creates a value of that variant type.
