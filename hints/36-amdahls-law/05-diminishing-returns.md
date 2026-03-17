# Hint: Diminishing Returns

Receive 4 results, sum them, then do the aggregation sleep:
```seq
dup chan.receive drop
over chan.receive drop i.+
over chan.receive drop i.+
over chan.receive drop i.+
nip nip

10 time.sleep-ms
```

Notice: 4 workers don't give 4x speedup because the serial overhead (setup + aggregation = 20ms) is fixed. Total ~ 30ms vs 60ms serial = ~2x, not 4x.
