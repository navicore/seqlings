# Maps (Dictionaries)

Maps are key-value stores that let you associate values with keys for fast lookup. In Seq, maps are created with `map.make` and manipulated with the `map.*` family of operations.

## Creating Maps

```seq
map.make    # Stack: ( Map )
```

Creates an empty map.

## Basic Operations

### Setting Values
```seq
my-map "name" "Alice" map.set    # Associates "Alice" with key "name"
```

### Getting Values
```seq
my-map "name" map.get    # Stack: ( "Alice" )
```

### Safe Get (with default)
```seq
my-map "missing" "default" map.get-or    # Returns "default" if key missing
```

### Checking Keys
```seq
my-map "name" map.contains?    # Stack: ( true )
```

### Removing Keys
```seq
my-map "name" map.remove    # Removes the key-value pair
```

## Inspection Operations

```seq
my-map map.size      # Number of key-value pairs
my-map map.keys      # List of all keys
my-map map.values    # List of all values
my-map map.empty?    # true if map has no entries
```

## Stack Effects

Most map operations consume the map and return a modified map:
```
map.set ( Map Key Value -- Map )
map.remove ( Map Key -- Map )
```

Query operations return results:
```
map.get ( Map Key -- Value )
map.size ( Map -- Int )
```

## Exercises in This Section

1. **basics** - Creating maps and setting values
2. **get** - Retrieving values from maps
3. **get-safe** - Handling missing keys gracefully
4. **remove** - Removing entries from maps
5. **inspection** - Keys, values, and size
6. **combine** - Building a complete key-value workflow
