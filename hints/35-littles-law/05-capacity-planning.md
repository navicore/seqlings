# Hint: Capacity Planning

Call `spawn-worker` 4 times, send 4 items, receive 4 results:
```seq
spawn-worker
spawn-worker
spawn-worker
spawn-worker

over 10 swap chan.send drop
over 20 swap chan.send drop
over 30 swap chan.send drop
over 40 swap chan.send drop

dup chan.receive drop
over chan.receive drop i.+
over chan.receive drop i.+
over chan.receive drop i.+
nip nip
```

With 4 workers and 4 items (20ms each), all process in parallel: ~20ms total vs ~80ms with 1 worker. L = λW tells us we need λ×W workers to maintain throughput.
