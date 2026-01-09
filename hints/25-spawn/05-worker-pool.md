# Hint: Worker Pool

Receive both results from the results channel. Remember `chan.receive` returns `(value Bool)`:
```seq
# Stack: ( work results )
dup chan.receive drop   # ( work results val1 )
over chan.receive drop  # ( work results val1 val2 )
i.+                     # ( work results sum )
nip nip                 # ( sum )
```
