# Hint: Creating Channels

`chan.make` creates a channel and leaves it on the stack. Stack effect: `( -- Channel )`.

The shape of this exercise: create the channel, send a value into it, receive the value back. Two complications make it more than three tokens long:

1. **Both `chan.send` and `chan.receive` consume the channel.** If you send a value and then want to receive from the same channel, you need TWO copies of the channel reference. A `dup` right after `chan.make` solves this.
2. **`chan.send` returns a Bool** (true on success, false if the channel is closed). The Bool needs to be dropped before the next operation. Same for `chan.receive`, which returns `( value Bool )` — both the value AND a status.

So the recipe is: make and dup the channel, send (drop the success Bool), receive (drop its success Bool, leaving just the value on top). The `42 swap` dance is because `chan.send`'s stack effect is `( value chan -- Bool )` — value below, channel on top. Whether you write `42 swap` or push 42 in a different order depends on which copy of the channel you've kept on top.

## Why the success Bool?

A channel can be closed (next exercise). Sending to a closed channel must fail somehow — Seq chose to return false rather than panic, so the caller can react. For these exercises where the channel is brand-new and never closed, the Bool is always true and `drop`ping it is safe.
