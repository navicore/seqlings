# Hint: HTTP Error Handling

Each classifier is a half-open range check: `code >= lower AND code < upper`. `dup` to copy the code, do the lower bound, `swap` to bring the other copy up for the upper bound, `and` the two Bools.

`describe-status` is a nested `if` cascade — one branch per class.

## Solution

```seq
: is-success? ( Int -- Bool )
    dup 200 i.>= swap 300 i.< and
;

: is-client-error? ( Int -- Bool )
    dup 400 i.>= swap 500 i.< and
;

: is-server-error? ( Int -- Bool )
    dup 500 i.>= swap 600 i.< and
;

: describe-status ( Int -- String )
    dup is-success? [
        drop "success"
    ] [
        dup is-client-error? [
            drop "client-error"
        ] [
            is-server-error? [
                "server-error"
            ] [
                "unknown"
            ] if
        ] if
    ] if
;
```

## How the Range Check Works

For `is-success?` with input `200`:

1. `dup` → `( 200 200 )`
2. `200 i.>=` → `( 200 true )` (top copy compared, leaves the lower copy)
3. `swap 300 i.<` → `( true true )` (lower copy compared against 300)
4. `and` → `( true )`

## Why the Nested `if` in `describe-status`?

Each `is-*?` consumes the int, so we `dup` before checking — then the branch that *matches* drops the duplicate and pushes the label; the branch that *doesn't* falls through with the int still on the stack for the next check. The deepest branch lets `is-server-error?` consume the last copy directly.
