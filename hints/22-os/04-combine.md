# Hint: OS Operation Patterns

## Solution

```seq
: get-home-or-default ( -- String )
    "HOME" os.env-has?
    [ "HOME" os.env-get ]
    [ "/tmp" ]
    if-else
;
```
