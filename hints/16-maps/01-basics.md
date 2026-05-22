# Hint: Map Basics

The stub already creates an empty map with `map.make`. Your job is to add one entry: key `"fruit"`, value `"apple"`.

`map.set` is the builder. Its stack effect is `( Map Key Value -- Map )` — it takes the map you're building on, the key, the value, and returns the updated map. So after `map.make`, push the key, push the value, call `map.set`. The result lands back on the stack as the updated map, ready for the `map.size` assertion that follows.

## Why `map.set` returns the map

Each `map.set` puts the updated map right back on the stack, so you can chain across multiple entries: `map.make ... map.set ... map.set ...`. You'll see exactly that in 06-combine when we build a 3-field profile. Functional update — no mutation, no aliasing surprises.
