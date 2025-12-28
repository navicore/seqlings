# Hint: Closing Connections

Always close sockets when done:

```seq
client tcp.close   # Close client connection
server tcp.close   # Close server when shutting down
```
