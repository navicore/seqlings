# Hint: Getting Current Time

## Solution

```seq
: time-is-positive ( -- Bool )
    time.now 0 >
;
```
