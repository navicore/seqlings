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
my-map "name" map.get    # Stack: ( "Alice" true )
```

`map.get` returns the value AND a Bool indicating whether the key was found. When you know the key exists, drop the Bool:

```seq
my-map "name" map.get drop    # Stack: ( "Alice" )
```

### Safe Get (handle missing keys)

There is no `map.get-or` builtin — branch on the Bool that `map.get` already returns:

```seq
my-map "missing" map.get
[ ]                       # found: value is on the stack
[ drop "default" ]        # missing: drop placeholder, push default
if
```

### Checking Keys
```seq
my-map "name" map.has?    # Stack: ( true )
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
map.set    ( Map Key Value -- Map )
map.remove ( Map Key -- Map )
```

Query operations:
```
map.get    ( Map Key -- Value Bool )
map.has?   ( Map Key -- Bool )
map.size   ( Map -- Int )
map.keys   ( Map -- List )
map.values ( Map -- List )
map.empty? ( Map -- Bool )
```

## Value Types

Maps are homogeneous in their value type — every entry must hold the same type. To store mixed types, wrap values in a Variant (chapter 14).

## Exercises in This Section

1. **basics** - Creating maps and setting values
2. **get** - Retrieving values from maps (and the Bool flag)
3. **get-safe** - Branching on the found-Bool for safe access
4. **remove** - Removing entries from maps
5. **inspection** - Keys, values, and size
6. **combine** - Building a complete key-value workflow
