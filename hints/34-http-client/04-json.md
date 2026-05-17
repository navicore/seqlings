# Hint: Working with JSON APIs

`api-success?` combines two checks: `ok=true` AND `status=200`. `get-json-body` pulls the `body` String and hands it to `json-parse`, which already returns `( Variant Bool )` — the exact shape the word advertises.

## Solution

```seq
include std:json

: api-success? ( Variant -- Bool )
    dup "ok" map.get drop
    swap "status" map.get drop
    200 i.= and
;

: get-json-body ( Variant -- Variant Bool )
    "body" map.get drop
    json-parse
;
```

## How `api-success?` Works

Starting stack: `( map )`.

1. `dup` → `( map map )`
2. `"ok" map.get drop` → `( map ok-bool )` (consumes the top copy)
3. `swap` → `( ok-bool map )` — bring the other copy up
4. `"status" map.get drop` → `( ok-bool status-int )`
5. `200 i.=` → `( ok-bool status-eq-200 )`
6. `and` → `( both-true )`

## Why `json-parse` Falls Through Cleanly

`json-parse ( String -- Variant Bool )` matches the shape of `get-json-body`'s return — so once `"body"` is on the stack, `json-parse` is the last call. On success the Variant is real and the Bool is true; on failure both are placeholders and the Bool is false, which the caller checks with `test.assert`.
