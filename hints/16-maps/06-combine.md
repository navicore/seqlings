# Hint: Building a Map Workflow

Maps in Seq are homogeneous in their value type. This profile uses Strings for every field (name, email, city), so `make-profile` takes three Strings.

## make-profile's plumbing

The stack going in is `( name email city )` — city on top. You need to call `map.set` three times with key/value pairs `"name"/name`, `"email"/email`, `"city"/city`. Each call consumes `( Map Key Value -- Map )`, so the map keeps coming back on top for the next call.

The challenge: each iteration the next value is buried under the map you're building, so you have to lift it. Push the key, then a single `rot` brings the buried value to the top — leaving `( Map Key Value )` exactly right for `map.set`.

That's three repetitions of `<key> rot map.set`. Process city first (it's on top and the most accessible), then email, then name. Same shape each iteration, different key.

## update-city

`update-city ( Map String -- Map )` is much simpler — a single `map.set`. You have Map and the new city value; `map.set` wants `( Map Key Value )`. Push the literal `"city"` key, get it under the new value with `swap`, call `map.set`.

## Why `rot` is the right reach

`rot` brings whatever's third from top to the top, leaving the order of the other two unchanged. For the pattern "I have a value buried under the map, I just pushed a key, I want `( Map Key Value )`," that's `rot` every time. Worth recognizing as an idiom; you'll see it in chapters that need to update a state record.

## Trace through make-profile

Stack walks for `( "Alice" "alice@example.com" "Portland" )`:

```
                          ( "Alice" "alice@example.com" "Portland" )
map.make            →     ( "Alice" "alice@example.com" "Portland" map )
"city" rot          →     ( "Alice" "alice@example.com" map "city" "Portland" )
map.set             →     ( "Alice" "alice@example.com" map' )
```

`map'` has `city: "Portland"`. Two more iterations finish the job.
