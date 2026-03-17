# Hint: Serial Baseline

After sending items, stack is `( results start )`. Receive 3 results then measure time:
```seq
rot dup chan.receive drop drop   # receive result 1
dup chan.receive drop drop       # receive result 2
dup chan.receive drop drop       # receive result 3
drop                             # drop results channel
time.now swap i.-                # elapsed = now - start
```
