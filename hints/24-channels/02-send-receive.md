# Hint: Send and Receive

Channels are FIFO — first value in is first value out. Send 10, then 20, then receive twice, and you get 10 first, 20 second. The assertion order in the test reflects that: it expects 20 on top (received last) and 10 below.

The mechanical challenge is keeping the channel reference alive across multiple operations. Each `chan.send` and `chan.receive` consumes the channel. For this exercise — two sends and two receives — you need four copies of the channel reference (or really, a `dup` before each operation except possibly the last).

The shape:

1. Make the channel.
2. For each send: `dup` the channel, push the value, `swap` to get the value below the channel (since `chan.send` wants `( value chan )`), call `chan.send`, drop the success Bool.
3. For each receive: `dup` (or `swap`, if the channel is buried), call `chan.receive`, drop the success Bool. The received value lands on the stack.

After four such operations the assertions consume the two received values.

## Why FIFO?

Channels model a queue, not a stack. If they were LIFO (last-in-first-out), they'd be a different abstraction — strands acting like callers / callees rather than producers / consumers. Queues are the right model for "I'm producing values, someone else is consuming them at their own pace," which is the most common use of channels.
