# Hint: Measuring Duration

## Solution

```seq
: elapsed-is-non-negative ( -- Bool )
    time.now
    time.now swap i.-
    0 >=
;
```
