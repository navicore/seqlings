# Hint: Measure λ (Throughput)

Bracket the whole send-receive block with `time.now`. Use `2 pick` to reach the work channel for repeated sends, `1 pick` to reach results for repeated receives. Both are non-destructive copies.

```seq
time.now                           # ( w r start )

2 pick 10 swap chan.send drop      # send each item via a 2-pick copy of w
2 pick 20 swap chan.send drop
2 pick 30 swap chan.send drop
2 pick 40 swap chan.send drop

1 pick chan.receive drop drop      # receive 4 results via 1-pick copies of r
1 pick chan.receive drop drop
1 pick chan.receive drop drop
1 pick chan.receive drop drop

time.now swap i.-                  # ( w r elapsed )
nip nip                            # ( elapsed )
```

Why `< 25_000us`? Service time per worker is 10ms = 10_000us. Serial execution of 4 items would be ~40_000us. Anything under 25_000us proves the workers really overlapped — that's λ scaling with K.
