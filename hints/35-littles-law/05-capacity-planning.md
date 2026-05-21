# Hint: Capacity Planning

Spawn 4 workers (K from `λ_target × W = 200 × 0.020 = 4`), bracket with `time.now`, then convert elapsed-microseconds back into items-per-second.

```seq
spawn-worker
spawn-worker
spawn-worker
spawn-worker

time.now                           # ( w r start )

10 3 pick chan.send drop           # push value, pick w (depth 3), send
20 3 pick chan.send drop
30 3 pick chan.send drop
40 3 pick chan.send drop

1 pick chan.receive drop drop      # nothing pushed, so r at depth 1
1 pick chan.receive drop drop
1 pick chan.receive drop drop
1 pick chan.receive drop drop

time.now swap i.-                  # ( w r elapsed_us )
4 1000000 i.* swap i./ drop        # ( w r λ ) — N×1_000_000/elapsed_us
nip nip                            # ( λ )
```

Why `N × 1_000_000` first? Integer math. `elapsed_us` is on the order of 20_000 and N is 4. `4 / 20_000` rounds to 0. Scaling N up by 10⁶ first gives a meaningful integer items-per-second.

The general formula: `K = ceil(λ_target × W_seconds)`. Same identity, applied backward — pick K from your target, instead of computing the actual L from a fixed K.
