# Hint: Chaining keep

Each `keep` computes something but preserves the value for the next:
```seq
4
[ dup i.* ] keep           # square: ( 16 4 )
[ dup dup i.* i.* ] keep   # cube: ( 16 64 4 )
```

For the cube, `dup dup i.* i.*` computes `4 * 4 * 4 = 64`.
