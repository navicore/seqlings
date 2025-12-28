# Hint: list.map

Apply a transformation to every element.

## Solution

```seq
: double-all ( List -- List )
    [ dup i.add ] list.map
;
```

Or equivalently:
```seq
: double-all ( List -- List )
    [ 2 i.multiply ] list.map
;
```

## The map Pattern

map is everywhere:
- JavaScript: `arr.map(x => x * 2)`
- Python: `list(map(lambda x: x * 2, lst))`
- Seq: `list [ 2 i.multiply ] list.map`

Same concept, different syntax.
