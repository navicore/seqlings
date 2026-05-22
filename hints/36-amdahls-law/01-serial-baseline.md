# Hint: Serial Baseline

At the point your code runs:

- The data stack is `( work results )` — both channels still live.
- The aux stack has the `start` timestamp on it (the prose-hinted `time.now >aux` stashed it before the sends).

What's left:

1. **Receive three results.** Each receive uses `dup chan.receive drop drop` — the `dup` keeps the results channel alive, `chan.receive` produces `( value Bool )`, and the two `drop`s discard both since we don't care about the values for a timing test, only that they completed. Three receives in a row.
2. **Drop both channel references** (`drop drop`) — you're done with them.
3. **Push `time.now` and pull `start` back from aux** with `aux>`. The data stack now has `( now start )`.
4. **Subtract** with `i.-` to get elapsed microseconds (or whatever unit `time.now` returns in your build — see ch 23 for the answer).

The do-not-edit assertion expects `elapsed >= 30` — for three 10ms sleeps in series, that holds easily.

## Why the aux stack here

You COULD keep `start` on the data stack between the sends and receives, but it would be in the way for every send and every receive — you'd be juggling three items on every operation. The aux stack is the right tool for "stash this value, get it out of my way, recover it after the busy work." Chapter 02's `>aux` / `aux>` exercise covered the mechanics.
