# Hint: Measuring a Duration

Two `time.nanos` calls leave `( before after )` on the stack —
no swap needed before `i.<=`, since the desired comparison is
`before <= after`.

## Solution

```seq
: monotonic-non-decreasing? ( -- Bool )
    time.nanos
    time.nanos
    i.<=
;
```
