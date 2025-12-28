# Hint: Square Roots

## Solution

```seq
: test-sqrt-100 ( -- )
    100.0 fmath.sqrt
    10.0 f.= test.assert
;

: hypotenuse ( Float Float -- Float )
    dup f.* swap dup f.* f.+ fmath.sqrt
;
```

For hypotenuse: square each, add, take square root.
