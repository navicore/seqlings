# Hint: Return Value

```seq
: compute-in-strand ( -- Int )
    chan.make
    dup [ 10 5 i.* swap chan.send drop ] strand.spawn
    drop
    chan.receive drop
;
```
