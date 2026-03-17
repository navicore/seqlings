# Hint: Measure Latency

After sending the work item, your stack has `( results start )`. Receive from the results channel, then measure time:
```seq
swap dup chan.receive drop drop   # receive result, drop value ( start results )
drop                              # ( start )
time.now swap i.-                 # ( elapsed )
```
