# Time Operations

Seq provides time-related operations through the `time.*` namespace.

## Getting Current Time

```seq
time.now           # Current timestamp in milliseconds
time.now-secs      # Current timestamp in seconds
```

## Sleeping

```seq
1000 time.sleep    # Sleep for 1000 milliseconds (1 second)
```

## Formatting

```seq
time.now time.format-iso    # ISO 8601 format string
```

## Measuring Duration

```seq
time.now
do-some-work
time.now swap i.subtract    # Elapsed time in ms
```

## Exercises in This Section

1. **now** - Getting current time
2. **sleep** - Pausing execution
3. **measure** - Measuring durations
4. **combine** - Time-based patterns
