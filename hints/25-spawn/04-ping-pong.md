# Hint: Ping-Pong

The stub already has everything except the final receive-and-cleanup step. At the point your code runs, the stack is `( ping pong )` — the ping channel below, the pong channel on top, and the worker strand has just sent `1` into pong.

What's left:

1. **Receive from pong.** It's already on top, so `chan.receive` is the next call. That consumes the pong channel and pushes `( value flag )`.
2. **Drop the success flag** so you're left with `( ping value )`.
3. **`nip`** discards the second-from-top item, which is the leftover ping reference you no longer need. That leaves the received value alone on top for the assertion.

Three tokens total: `chan.receive drop nip`.

## Why two channels for a ping-pong

You could imagine doing this with one channel, but then both strands would be sending AND receiving on the same conduit, and you'd need careful turn-taking. Two channels — one for each direction — makes the protocol explicit: ping flows one way, pong the other. This is the conventional shape for request/response over channels and you'll see it again in 26-tcp.
