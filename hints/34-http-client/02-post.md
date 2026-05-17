# Hint: HTTP POST Requests

The content-type words are pure string constants. `make-json-body` is the real exercise: assemble `{"name":"...","age":N}` by stringifying the int, then concatenating the literal pieces.

## Solution

```seq
: json-content-type ( -- String )
    "application/json"
;

: form-content-type ( -- String )
    "application/x-www-form-urlencoded"
;

: make-json-body ( String Int -- String )
    # ( name age -- json )
    int->string
    swap
    "{\"name\":\"" swap string.concat
    "\",\"age\":" string.concat
    swap string.concat
    "}" string.concat
;
```

## How `make-json-body` Works

Starting stack: `( "Alice" 30 )`.

1. `int->string` → `( "Alice" "30" )`
2. `swap` → `( "30" "Alice" )`
3. `"{\"name\":\""` push → `( "30" "Alice" "{\"name\":\"" )`
4. `swap string.concat` → `( "30" "{\"name\":\"Alice" )`
5. `"\",\"age\":" string.concat` → `( "30" "{\"name\":\"Alice\",\"age\":" )`
6. `swap string.concat` → `( "{\"name\":\"Alice\",\"age\":30" )`
7. `"}" string.concat` → `( "{\"name\":\"Alice\",\"age\":30}" )`

The age (`"30"`) is stashed on the bottom of the stack while we build the prefix, then pulled forward with the last `swap`.
