# Hint: list.map

Apply a transformation to every element.

## Solution

```seq
: double-all ( Variant -- Variant )
    [ dup i.+ ] list.map
;
```

Or equivalently:
```seq
: double-all ( Variant -- Variant )
    [ 2 i.* ] list.map
;
```

## The map Pattern

map is everywhere:
- JavaScript: `arr.map(x => x * 2)`
- Python: `list(map(lambda x: x * 2, lst))`
- Seq: `list [ 2 i.* ] list.map`

Same concept, different syntax.
