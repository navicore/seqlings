# Hint: Return Value

```seq
: compute-in-strand ( -- Int )
    chan.make
    dup [ 10 5 i.* chan.send ] spawn
    drop
    chan.receive
;
```
