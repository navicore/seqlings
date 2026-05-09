# Hint: Reading Environment Variables

`os.getenv` returns *two* values. After `swap os.getenv` the
stack is `( fallback value present? )`. The Bool drives an `if`,
and the two branches both end with a single string on top:

- present: drop the fallback, keep the value (`nip`)
- missing: drop the empty value, keep the fallback (`drop`)

## Solution

```seq
: getenv-or ( name fallback -- String )
    swap os.getenv
    [ nip ]
    [ drop ]
    if
;
```
