# Hint: Reading and Writing Data

The two pieces you fill in:

```seq
dup net.tcp.read
[ # success: ( server client data )
  "hello" test.assert-eq-str       # consumes data and "hello"
]
[ # failure: ( server client data )
  drop                              # drop the empty data string
]
if
```

`test.assert-eq-str ( String String -- )` takes two strings off the
stack and asserts they're equal. The read pushed `data` on top, so
pushing `"hello"` makes them adjacent in the order
`( data "hello" )`. `assert-eq-str` consumes both.

## Why `dup` before read

`net.tcp.read ( Socket -- String Bool )` consumes the socket from
the stack — but the underlying handle stays open. We still need to
close the client at the end (in the "Do not edit below" section),
so we `dup` the socket *before* read to keep one copy for the close.

## Why not check the write Bool in `connector`

We do, in spirit — the `# seq:allow(unchecked-tcp-write)` comment
on `connector` acknowledges that we're choosing to ignore the
write's Bool in this test. The lint exists because production code
that drops the write Bool will silently lose data on a failed write.
The next exercise (04) shows what happens when you actually do check
these Bools.
