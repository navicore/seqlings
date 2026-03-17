# Hint: Queue Depth

After sending 3 items, receive 3 results. Stack has `( results start )` after sends:
```seq
rot dup chan.receive drop        # ( start results val1 )
over chan.receive drop i.+       # ( start results sum )
over chan.receive drop i.+       # ( start results total )
nip nip                          # ( start total )
```

The sum of three items (value 1, each +1 = 2) is 6.
