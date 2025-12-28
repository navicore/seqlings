# Hint: Stack Effects

You need to produce BOTH the sum and the product. That means preserving both inputs.

## Solution

```seq
: sum-and-product ( Int Int -- Int Int )
    2dup i.multiply    # Stack: ( a b product )
    rot rot i.add      # Stack: ( product sum )
;
```

Wait, that's not quite right for the test. Let me trace more carefully:

```seq
: sum-and-product ( Int Int -- Int Int )
    2dup i.add      # ( a b sum )
    rot rot         # ( sum a b )
    i.multiply      # ( sum product )
    swap            # ( product sum )
;
```

Hmm, the test expects sum on top. Let me check the assertion order...

The test does `7 test.assert-eq` then `12 test.assert-eq`, meaning it pops 7 first (sum) then 12 (product). So we need `( product sum )` - product below, sum on top.

```seq
: sum-and-product ( Int Int -- Int Int )
    2dup i.multiply   # ( a b product )
    -rot              # ( product a b )
    i.add             # ( product sum )
;
```

Or more simply:
```seq
: sum-and-product ( Int Int -- Int Int )
    2dup i.multiply
    rot rot i.add
;
```
