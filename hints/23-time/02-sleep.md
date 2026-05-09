# Hint: Sleeping

`time.sleep-ms` consumes its argument and leaves nothing on the
stack. Push the duration, call the word, and then push `true`
for the test to assert.

## Solution

```seq
: tiny-pause ( -- Bool )
    5 time.sleep-ms
    true
;
```
