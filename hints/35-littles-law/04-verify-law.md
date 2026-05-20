# Hint: Verify L = λW

Each receive needs to record W_i = `time.now - start`, then fold it into a running sum. Pattern (one line per receive):

```seq
2 pick chan.receive drop drop time.now 2 pick i.- swap i.+
```

Walking through that line, starting from `( w r start sum )`:

- `2 pick` copies r → receive consumes the copy → `drop drop` discards value+Bool → back to `( w r start sum )`
- `time.now 2 pick i.-` pushes the receive-time, copies start, subtracts → `( w r start sum W_i )`
- `swap i.+` folds W_i into sum → `( w r start sum' )`

Three of those lines and you have sum_W. Then:

```seq
time.now 2 pick i.-     # ( w r start sum_W elapsed )
i./ drop                # ( w r start L )       — i./ returns ( quotient Bool ), drop the Bool
nip nip nip             # ( L )
```

The algebra: `sum_W / elapsed = (N × W_avg) / elapsed = (N/elapsed) × W_avg = λ × W_avg = L`. One division does the whole identity.
