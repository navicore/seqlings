# Hint: Echo Server Pattern

The echo handler reads and writes back the same data:

```seq
: echo-handler ( socket -- )
    dup tcp.read      # Read data
    over tcp.write    # Write it back
    tcp.close         # Close connection
;
```
