# Hint: Verify Little's Law

Same pattern as throughput - receive 4 results and sum:
```seq
dup chan.receive drop
over chan.receive drop i.+
over chan.receive drop i.+
over chan.receive drop i.+
nip nip
```

With 2 workers processing 20ms each, 4 items complete in ~40ms instead of ~80ms. That's Little's Law in action: doubling workers (L) doubles throughput (λ) for the same latency (W).
