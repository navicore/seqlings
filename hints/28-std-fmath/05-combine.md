# Hint: Mathematical Formulas

## Circle Area

```seq
: circle-area ( Float -- Float )
    dup f.* fmath.pi f.*
;
```

## Degrees to Radians

```seq
: degrees-to-radians ( Float -- Float )
    fmath.pi f.* 180.0 f./
;
```

## Euclidean Distance

```seq
: distance ( Float Float Float Float -- Float )
    # Stack: x1 y1 x2 y2
    rot f.- dup f.*    # (y2-y1)^2
    rot rot f.- dup f.*  # (x2-x1)^2
    f.+ fmath.sqrt
;
```
