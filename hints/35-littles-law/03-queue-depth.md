# Hint: Measure L (Queue Depth)

Maintain an in-flight counter on the stack: increment after each `chan.send`, decrement after each `chan.receive`. `dup` it at peak to save the snapshot.

```seq
0                                                # ( w r in_flight=0 )

2 pick 1 swap chan.send drop 1 i.+               # send + increment
2 pick 2 swap chan.send drop 1 i.+
2 pick 3 swap chan.send drop 1 i.+
2 pick 4 swap chan.send drop 1 i.+               # ( w r 4 )

dup                                              # ( w r peak=4 in_flight=4 ) — save peak

2 pick chan.receive drop drop 1 i.-              # receive + decrement
2 pick chan.receive drop drop 1 i.-
2 pick chan.receive drop drop 1 i.-
2 pick chan.receive drop drop 1 i.-              # ( w r peak 0 )

drop                                             # drop trailing zero
nip nip                                          # ( peak )
```

Why is `2 pick` the right reach? With `( w r counter )` on the stack, the depths are 0=counter, 1=r, 2=w — so `2 pick` copies w for the send. After `dup`-saving the peak, the stack becomes `( w r peak counter )` so r is at depth 2 — same `2 pick` reaches it.
