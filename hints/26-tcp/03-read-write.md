# Hint: Reading and Writing Data

Read from a socket:
```seq
client net.tcp.read   # ( socket -- string )
```

Write to a socket:
```seq
"Hello" client net.tcp.write   # ( string socket -- )
```
