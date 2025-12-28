# Hint: Option Type

## Solution

```seq
: test-some-not-none ( -- )
    42 Some None?
    false test.assert-eq
;
```

`Some` and `None` are mutually exclusive variants.
