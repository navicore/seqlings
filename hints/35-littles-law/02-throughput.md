# Hint: Throughput

Receive 4 values from the results channel and sum them. Use `dup`/`over` to keep the channel on stack:
```seq
dup chan.receive drop           # ( work results val1 )
over chan.receive drop i.+      # ( work results sum2 )
over chan.receive drop i.+      # ( work results sum3 )
over chan.receive drop i.+      # ( work results sum4 )
nip nip                         # ( total )
```
