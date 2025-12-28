# Hint: Variant Basics

## variant.tag

Each variant has a tag index (0, 1, 2, ...) corresponding to its position in the definition.

```seq
union TrafficLight { Stop, Go, Caution }
# Stop = tag 0, Go = tag 1, Caution = tag 2
```

The compiler auto-generates `Make-Stop`, `Make-Go`, `Make-Caution` constructors.

## Solution

```seq
: test-create-go ( -- )
    Make-Go variant.tag
    1 test.assert-eq   # Go is at index 1
;
```
