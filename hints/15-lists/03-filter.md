# Hint: list.filter

The quotation is a predicate — it returns true/false for each
element. `i.modulo` leaves `( remainder success )`, so `drop`
the success flag before comparing.

Also note: `List` isn't a type in the strict 7.0 type system —
collections are `Variant` underneath.

## Solution

```seq
: evens-only ( Variant -- Variant )
    [ 2 i.modulo drop 0 i.= ] list.filter
;
```

## Understanding the Predicate

For each element:
1. `2 i.modulo` — compute `element mod 2`, leaves `(remainder true)`
2. `drop` — drop the success flag
3. `0 i.=` — is the remainder zero?
4. If true, element is kept; if false, discarded
