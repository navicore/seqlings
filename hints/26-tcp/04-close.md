# Hint: Closing Connections

Always close sockets when done:

```seq
client net.tcp.close   # Close client connection
server net.tcp.close   # Close server when shutting down
```
