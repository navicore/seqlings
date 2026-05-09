# Time Operations

Seq exposes three time words. That's the whole API.

```
time.now       ( -- Int )    Unix timestamp in seconds (wall clock)
time.nanos     ( -- Int )    monotonic time in nanoseconds
N time.sleep-ms              sleep for N milliseconds
```

## Two clocks, two jobs

`time.now` and `time.nanos` are not interchangeable.

- `time.now` is **wall-clock seconds** since the Unix epoch. Good
  for "what time is it?" — timestamps you'd write to a log,
  display to a user, or compare against another wall-clock value.
  Wall clocks can jump backwards when NTP adjusts the system
  clock, so don't use them to measure how long something took.

- `time.nanos` is a **monotonic** counter that only moves
  forward. It has no defined zero point — the value on its own
  is meaningless. You read it twice and subtract:

  ```seq
  time.nanos       # before
  do-some-work
  time.nanos       # before after
  swap i.-         # after - before, in nanoseconds
  ```

  This is the right tool for measuring durations.

## Sleeping

```seq
1000 time.sleep-ms    # pause for 1 second
```

Sleep is "*at least* N milliseconds" — the OS may schedule you
back later than that, but never sooner. There's no `time.sleep`
or `time.sleep-secs`; multiply by 1000 if you want seconds.

## Stack effects

| Word | Stack Effect | Notes |
|------|--------------|-------|
| `time.now`        | `( -- Int )` | seconds since 1970-01-01 UTC |
| `time.nanos`      | `( -- Int )` | monotonic ns; only deltas are meaningful |
| `time.sleep-ms`   | `( Int -- )` | blocks the current sequence |

## Exercises in This Section

1. **01-now** — read the wall clock with `time.now`
2. **02-sleep** — pause execution with `time.sleep-ms`
3. **03-measure** — measure a duration with `time.nanos`
4. **04-combine** — sleep and measure how long it actually took
