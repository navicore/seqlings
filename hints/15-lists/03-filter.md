# Hint: list.filter

The quotation is a predicate - it returns true/false for each element.

## Solution

```seq
: evens-only ( List -- List )
    [ 2 i.mod 0 = ] list.filter
;
```

## Understanding the Predicate

For each element:
1. `2 i.mod` - compute element mod 2
2. `0 =` - is it zero?
3. If true, element is kept; if false, discarded
