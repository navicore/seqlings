# Hint: Return Value

The quotation captures the channel by closure when `[` is built,
so the channel on the stack stays available for main to receive
on. No `dup` needed.

## Solution

```seq
: compute-in-strand ( -- Int )
    chan.make
    [ 10 5 i.* swap chan.send drop ] strand.spawn
    drop                  # drop strand-id
    chan.receive drop     # ( 50 )
;
```
