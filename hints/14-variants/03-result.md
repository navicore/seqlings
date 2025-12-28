# Hint: Result Type

## Solution

```seq
: success ( Int -- Result )
    Ok
;

: failure ( String -- Result )
    Err
;
```

Just apply the constructor to wrap the value.
