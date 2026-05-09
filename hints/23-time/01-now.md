# Hint: Reading the Wall Clock

Push the timestamp, push 0, ask whether the first is greater
than the second. `i.>` is the integer greater-than (the bare `>`
isn't a builtin).

## Solution

```seq
: time-is-positive ( -- Bool )
    time.now 0 i.>
;
```
