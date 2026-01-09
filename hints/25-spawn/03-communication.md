# Hint: Two-Way Communication

The key insight: one channel can be used for both sending and receiving, just at different times.

After sending 21 to the worker, receive the doubled result:
```seq
chan.receive drop   # receive 42, drop the Bool: ( value )
```
