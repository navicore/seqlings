# Hint: HTTP GET Requests

Responses are Maps. Each accessor pulls a key with `map.get` and drops the lookup-success Bool (the test guarantees the keys exist). `response-body` is the only one with logic: return `""` if the response wasn't ok, otherwise return the body.

## Solution

```seq
: response-ok? ( Variant -- Bool )
    "ok" map.get drop
;

: response-body ( Variant -- String )
    dup response-ok? [
        "body" map.get drop
    ] [
        drop ""
    ] if
;

: response-status ( Variant -- Int )
    "status" map.get drop
;
```

## How `response-body` Works

`dup` keeps a copy of the map around while `response-ok?` consumes the duplicate to produce a Bool. The `if` then either reads `"body"` from the still-on-stack map, or drops the map and pushes `""`.
