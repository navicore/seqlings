# Chapter 34: HTTP Client

Seq provides built-in HTTP client functions for making web requests.

## HTTP Methods

```seq
http.get    ( String -- Map )                    # GET request
http.post   ( String String String -- Map )      # POST with body and content-type
http.put    ( String String String -- Map )      # PUT with body and content-type
http.delete ( String -- Map )                    # DELETE request
```

## Response Format

All HTTP functions return a Map with these keys:

| Key | Type | Description |
|-----|------|-------------|
| `ok` | Bool | true if request succeeded (2xx status) |
| `status` | Int | HTTP status code |
| `body` | String | Response body |
| `error` | String | Error message (on failure) |

## Example Usage

```seq
# Simple GET
"https://api.example.com/data" http.get
dup "ok" map.get drop
if
    "body" map.get drop io.write-line
else
    "error" map.get drop io.write-line
then

# POST JSON
"https://api.example.com/users"
"{\"name\":\"Alice\"}"
"application/json"
http.post
```

## Error Handling

Always check the `ok` field before using the response:

```seq
: fetch-data ( String -- String Bool )
    http.get
    dup "ok" map.get drop
    if
        "body" map.get drop true
    else
        "error" map.get drop false
    then
;
```

## Common Content Types

- `application/json` - JSON data
- `application/x-www-form-urlencoded` - Form data
- `text/plain` - Plain text

## Exercises

1. **01-get** - Making GET requests
2. **02-post** - Sending POST data
3. **03-errors** - Handling HTTP errors
4. **04-json** - Working with JSON APIs

Note: These exercises use httpbin.org for testing. Network access is required.
