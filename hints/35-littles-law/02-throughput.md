# Hint: Measure λ (Throughput)

Bracket the whole send-receive block with `time.now`. To send, push the value first and then `3 pick` the work channel above it; that lands `( value chan )` on top, exactly what `chan.send` wants. To receive, `1 pick` the results channel directly (nothing pushed above it). Both `pick`s are non-destructive copies.

```seq
time.now                           # ( w r start )

10 3 pick chan.send drop           # ( w r start 10 w ) → send → drop Bool
20 3 pick chan.send drop
30 3 pick chan.send drop
40 3 pick chan.send drop

1 pick chan.receive drop drop      # ( w r start r ) → receive → drop val+Bool
1 pick chan.receive drop drop
1 pick chan.receive drop drop
1 pick chan.receive drop drop

time.now swap i.-                  # ( w r elapsed )
nip nip                            # ( elapsed )
```

The pick-depth rule: on `( w r start )`, work is at depth 2 and results at depth 1. Pushing a value bumps each down by one, so for sends you reach for depth 3; receives push nothing first, so depth 1 is still results.

Why `< 25_000us`? Service time per worker is 10ms = 10_000us. Serial execution of 4 items would be ~40_000us. Anything under 25_000us proves the workers really overlapped — that's λ scaling with K.
