# Hint: Two-Way Communication

The worker captures both channels via closure. With
`( request response )` on the stack at `[`, the body sees
`( request response )` too. `swap` brings request to the top
for `chan.receive`; after doubling, `swap` again brings
response to the top for `chan.send`.

## Solution

```seq
: spawn-doubler ( -- Channel Channel )
    chan.make
    chan.make
    [
        swap chan.receive drop   # receive request: ( response val )
        2 i.*                    # ( response doubled )
        swap chan.send drop      # send doubled
    ] strand.spawn drop          # ( request response )
;
```
