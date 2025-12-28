# Hint: Reading and Writing Data

Read from a socket:
```seq
client tcp.read   # ( socket -- string )
```

Write to a socket:
```seq
"Hello" client tcp.write   # ( string socket -- )
```
