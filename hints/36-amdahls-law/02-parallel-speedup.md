# Hint: Parallel Speedup

Receive 4 results and sum them. Same pattern as the worker-pool exercises:
```seq
dup chan.receive drop
over chan.receive drop i.+
over chan.receive drop i.+
over chan.receive drop i.+
nip nip
```
