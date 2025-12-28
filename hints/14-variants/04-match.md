# Hint: Pattern Matching

## Solution

```seq
: unwrap-or ( Option Int -- Int )
    swap
    variant.match
        | Some -> [ swap drop ]
        | None -> [ ]
    end
;
```

In the `Some` case, we have the value and the default - drop the default.
In the `None` case, we just have the default - keep it.
