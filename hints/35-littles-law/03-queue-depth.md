# Hint: Measure L (Queue Depth)

Maintain an in-flight counter on the stack: increment after each `chan.send`, decrement after each `chan.receive`. `dup` it at peak to save the snapshot.

```seq
0                                                # ( w r in_flight=0 )

1 3 pick chan.send drop 1 i.+                    # push value, pick w, send, +1
2 3 pick chan.send drop 1 i.+
3 3 pick chan.send drop 1 i.+
4 3 pick chan.send drop 1 i.+                    # ( w r 4 )

dup                                              # ( w r peak=4 in_flight=4 ) — save peak

2 pick chan.receive drop drop 1 i.-              # pick r, receive, -1
2 pick chan.receive drop drop 1 i.-
2 pick chan.receive drop drop 1 i.-
2 pick chan.receive drop drop 1 i.-              # ( w r peak 0 )

drop                                             # drop trailing zero
nip nip                                          # ( peak )
```

Why those pick depths? With `( w r counter )` on the stack, depths are 0=counter, 1=r, 2=w. Pushing the value bumps everything down by one, so for sends w is at depth 3 — that's why `3 pick` comes *after* the value. Receives push nothing first, and during the receive phase the stack is `( w r peak counter )` — r is at depth 2, so `2 pick` reaches it.
