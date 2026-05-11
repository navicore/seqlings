# Hint: Square Roots

Square each leg, add the squares, take the square root.

## Solution

```seq
: hypotenuse ( Float Float -- Float )
    dup f.* swap dup f.* f.+ f.sqrt
;
```
