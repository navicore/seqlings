# Hint: Result Type

## Constructors

The constructors are auto-generated as `Make-Ok` and `Make-Err`:

```seq
: success ( Int -- IntResult )
    Make-Ok
;

: failure ( String -- IntResult )
    Make-Err
;
```

Just apply the constructor to wrap the value.

## Checking variants with match

```seq
: is-ok? ( IntResult -- Bool )
    match
        Ok { >value } -> drop true
        Err { >error } -> drop false
    end
;
```
