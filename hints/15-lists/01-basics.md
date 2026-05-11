# Hint: List Basics

Use `list.length` to count elements. Lists are created using `string.split`.

## Solution

```seq
: has-three-elements? ( Variant -- Bool )
    list.length 3 i.=
;
```
