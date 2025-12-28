# Hint: Return Value

```seq
: compute-in-strand ( -- Int )
    chan.make
    dup [ 10 5 i.multiply chan.send ] spawn
    drop
    chan.receive
;
```
