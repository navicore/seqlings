# Hint: Echo Server Pattern

The echo handler reads and writes back the same data:

```seq
: echo-handler ( socket -- )
    dup net.tcp.read      # Read data
    over net.tcp.write    # Write it back
    net.tcp.close         # Close connection
;
```
