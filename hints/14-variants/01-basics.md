# Hint: Variant Basics

## Using match

Use `match` to check which variant you have:

```seq
union TrafficLight { Stop Go Caution }

: is-stop? ( TrafficLight -- Bool )
    match
        Stop -> true
        Go -> false
        Caution -> false
    end
;
```

The compiler auto-generates `Make-Stop`, `Make-Go`, `Make-Caution` constructors.

## Solution

```seq
: is-go? ( TrafficLight -- Bool )
    match
        Stop -> false
        Go -> true
        Caution -> false
    end
;
```
