# Hint: Sleep and Measure

Read `time.nanos` once before the sleep and once after. Stash
the "before" reading underneath the target so the sleep word
finds the target on top:

```
( target-ms )
time.nanos        ( target-ms before )
swap              ( before target-ms )
time.sleep-ms     ( before )
time.nanos        ( before after )
swap i.-          ( after - before )
```

`a b i.-` is `a - b`, so the final `swap` flips `before after`
into `after before`.

## Solution

```seq
: sleep-elapsed-nanos ( Int -- Int )
    time.nanos
    swap
    time.sleep-ms
    time.nanos
    swap i.-
;
```
