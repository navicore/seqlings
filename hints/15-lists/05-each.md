# Hint: list.each

each is for side effects - it doesn't produce a new list.

## Solution

```seq
: print-doubled ( List -- )
    [ dup i.+ int->string io.write-line ] list.each
;
```

## each vs map

- `map`: transforms elements, returns new list
- `each`: executes for effect, returns nothing

Use each when you want to DO something (print, write to file) rather than COMPUTE something.
