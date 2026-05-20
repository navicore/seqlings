# Hint: Measure W (Latency)

Going in: `( results start )`. You need to receive on `results` (one-shot, the value doesn't matter), then take a fresh `time.now` and subtract `start`:

```seq
swap chan.receive drop drop      # ( start ) — value+Bool dropped, results consumed
time.now swap i.-                # ( W )
```

`time.now` is microseconds, not milliseconds — the elapsed value will be in the tens-of-thousands range (10ms ≈ 10_000us), which is why the test asserts `> 9_000us`.
