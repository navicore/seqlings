# Hint: keep

`keep` preserves the top value across the quotation's effect. The quotation transforms; `keep` restores the original on top afterward.

```seq
7 [ 1 i.+ ] keep
# quotation sees ( 7 ) → produces ( 8 )
# keep puts the original 7 back on top → ( 8 7 )
```

No `dup` is needed in the quotation when the transformation is unary — `keep` is the one doing the preservation, on the outside.

When you DO see `dup` inside a quotation under `keep` (later exercises will), that's because the operator inside the quotation needs two copies of the value — `[ dup i.* ]` for squaring, for example. The inner `dup` is feeding the binary `i.*`, not preserving anything for after `keep`.
