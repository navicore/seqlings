# Hint: keep

`keep` runs the quotation on the value but also preserves the original:
```seq
7 [ dup i.* ] keep
# dup i.* squares 7 → 49, keep restores 7 → ( 49 7 )
```

The quotation receives the value, so you need `dup` inside if you want to use it for multiplication.
