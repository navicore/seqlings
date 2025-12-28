# Hint: Mathematical Formulas

## Circle Area

```seq
: circle-area ( Float -- Float )
    dup f.multiply fmath.pi f.multiply
;
```

## Degrees to Radians

```seq
: degrees-to-radians ( Float -- Float )
    fmath.pi f.multiply 180.0 f.divide
;
```

## Euclidean Distance

```seq
: distance ( Float Float Float Float -- Float )
    # Stack: x1 y1 x2 y2
    rot f.subtract dup f.multiply    # (y2-y1)^2
    rot rot f.subtract dup f.multiply  # (x2-x1)^2
    f.add fmath.sqrt
;
```
