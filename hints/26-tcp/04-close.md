# Hint: Detecting Peer Close

The three-way read pattern:

```seq
dup net.tcp.read
[ # success branch — ( server client data )
  string.length 0 test.assert-eq      # peer closed → length must be 0
]
[ # failure branch — ( server client data )
  drop                                 # drop empty data string
  false test.assert                    # hard error → fail the test
]
if
```

Note that `test.assert-eq` here is the integer version, comparing the
length returned by `string.length` against `0`. It consumes both
integers from the stack, leaving `( server client )` ready for the
cleanup below.

## Why three outcomes from one Bool

`net.tcp.read`'s success Bool only distinguishes "kernel error"
(false) from "kernel happy" (true). Inside "kernel happy" there are
still two real outcomes:

- Bytes arrived → length > 0 → process them and keep reading.
- Peer closed → length == 0 → stop reading and clean up.

A naïve `if`-on-the-Bool that just keeps calling `read` on a closed
peer is the canonical TCP infinite-loop bug. The check is
`string.length 0 i.eq` — if the data is empty *and* the read
succeeded, the other end has hung up.

## What if the connector wrote data?

Then `string.length 0 test.assert-eq` would fail (the length would
be 5 for `"hello"`, not 0). That's the correct failure: this
exercise specifically tests the peer-closed-without-writing case.
The real `read-everything` loop alternates the two truthy cases —
keep reading until length-zero, then close.
