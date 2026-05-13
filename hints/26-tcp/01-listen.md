# Hint: Starting a TCP Listener

`net.tcp.listen ( Int -- Socket Bool )` always ends with a success
Bool — the same pattern as every other word in `net.tcp.*`. Use `if`
on that Bool to split the success and failure paths:

```seq
18261 net.tcp.listen
[ # success branch, stack: ( socket )
  net.tcp.close drop     # close the socket, drop close's success Bool
  true                    # this exercise's verdict
]
[ # failure branch, stack: ( socket-junk )
  drop                    # the socket on failure is meaningless
  false
]
if
test.assert
```

Why bother closing in a "just listen" exercise? Because the next
exercise reuses the same kernel — leaving the listener open here
would block the next test's bind on the same port (this exercise
uses port 18261; the next uses 18262, etc., so re-running by hand
also stays clean).
