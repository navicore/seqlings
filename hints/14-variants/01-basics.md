# Hint: Variant Basics

## variant.tag

Each variant has a tag index (0, 1, 2, ...) corresponding to its position in the definition.

```seq
variant TrafficLight
    | Stop     # tag 0
    | Go       # tag 1
    | Caution  # tag 2
end
```

## Solution

```seq
: test-create-go ( -- )
    Go variant.tag
    1 test.assert-eq   # Go is at index 1
;
```
