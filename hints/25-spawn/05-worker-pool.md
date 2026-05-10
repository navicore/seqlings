# Hint: Worker Pool

Receive twice from `results`, sum the two values, then `nip nip`
to drop the leftover work and results channels.

```seq
# Stack: ( work results )
dup chan.receive drop    # ( work results val1 )
over chan.receive drop   # ( work results val1 val2 )
i.+                      # ( work results sum )
nip nip                  # ( sum )
```
