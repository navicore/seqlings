# Hint: Parallel Speedup

The workers and sends are all done — your job is the receive-and-sum step at the end.

The stack at this point is `( work results )` (channels still on the stack from the sends). You need to receive four values from the results channel, add them together, and clean up so only the sum remains on top for the assertion.

The pattern is the same as the worker-pool exercises from chapter 25:

1. Receive the first value with `dup chan.receive drop` (dup the results channel so you can keep receiving, run the receive, drop its success flag). You now have one value below the work and results channels.
2. For each remaining receive, use `over chan.receive drop` to keep using the results channel without consuming it, and `i.+` immediately to fold into the running total.
3. After four receives, the stack is `( work results sum )`. Two `nip`s drop the channels and leave only the sum.

That last `nip nip` step is the bit that gets forgotten — easy to leave channel references on the stack and confuse the assertion. The test wants the sum on top with nothing else.

## Why this confirms speedup

The sum `11 + 21 + 31 + 41 = 104` is the marker that all four items round-tripped through both workers (each adds 1 to the item, so 10→11, 20→21, etc.). The exercise doesn't actually time anything — the parallelism is implied by the two-worker setup. The next exercise has you compute the actual speedup ratio.
