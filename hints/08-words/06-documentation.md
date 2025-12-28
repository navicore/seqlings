# Hint: Clamp

Clamp needs to handle three cases:
1. value < min → return min
2. value > max → return max
3. otherwise → return value

## Solution

```seq
: clamp ( Int Int Int -- Int )
    # Stack: ( value min max )
    rot             # ( min max value )
    dup rot         # ( min value value max )
    > if            # if value > max
        drop swap drop  # return max
    else
        dup rot     # ( max value value min )
        < if        # if value < min
            drop nip    # return min
        else
            nip nip     # return value
        then
    then
;
```

This is getting complex! An alternative using `cond`:

```seq
: clamp ( Int Int Int -- Int )
    rot rot 2dup    # Setup for comparisons
    cond
        2dup > -> drop drop drop swap drop  # value > max
        2dup < -> drop drop nip nip         # value < min
        otherwise -> drop drop nip drop     # in range
    end
;
```

The complexity here shows why clamping is usually a library function!
