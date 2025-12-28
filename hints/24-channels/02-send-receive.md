# Hint: Send and Receive

Channels are FIFO - first in, first out.

## Solution

```seq
chan.make
dup 10 chan.send
dup 20 chan.send
dup chan.receive   # Gets 10
swap chan.receive  # Gets 20
```
