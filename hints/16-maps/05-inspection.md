# Hint: Map Inspection

Four words for poking at maps without modifying them:

- `map.size` — Int, the number of entries.
- `map.empty?` — Bool, true if there are no entries.
- `map.keys` — list of all keys.
- `map.values` — list of all values.

Each test pushes a map (the stub already calls `make-test-map`) and applies one of these words. The bodies are one or two tokens each:

- **`test-size`** wants the count — one word.
- **`test-empty`** wants the Bool — one word.
- **`test-keys-count`** wants a count of how many keys, so it's two words in sequence: one to get the keys, then `list.length` to count them.

## The `"tmp" 0 map.set "tmp" map.remove` dance

In `test-empty`, the empty-test map is built with a temporary entry that's immediately removed. Why? `map.make` alone gives an empty map whose value type the type-checker can't infer. Setting and removing an Int entry pins the type to `( Map String Int )` so the empty-check has a concrete map to work on. Same outcome (empty map), no type-inference complaint.
