# Hint: Serial Fraction

After sending 2 items, receive results, then do the aggregation sleep:
```seq
dup chan.receive drop             # ( work results val1 )
over chan.receive drop i.+        # ( work results sum )
nip nip                           # ( sum )

10 time.sleep-ms                  # serial aggregation phase
```

The serial phases (setup + aggregation = 20ms) can never be parallelized. That's Amdahl's insight.
