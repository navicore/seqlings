# Hint: Spawning

At the marker the stack is `( chan )`. `chan.receive` consumes
the channel and returns ( value flag ); drop the flag and the
value is exactly what `42 test.assert-eq` needs.

## Solution

```seq
: test-basics ( -- )
    chan.make
    [ 42 swap chan.send drop ] strand.spawn
    drop                  # drop strand-id
    chan.receive drop     # ( 42 )
    42 test.assert-eq
;
```
