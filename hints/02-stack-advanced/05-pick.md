# Hint: pick

`pick` copies an element from a given depth (0-indexed from top).

## Understanding pick

```seq
10 20 30 40  0 pick    # Copies top (40)      Stack: ( 10 20 30 40 40 )
10 20 30 40  1 pick    # Copies second (30)   Stack: ( 10 20 30 40 30 )
10 20 30 40  2 pick    # Copies third (20)    Stack: ( 10 20 30 40 20 )
10 20 30 40  3 pick    # Copies fourth (10)   Stack: ( 10 20 30 40 10 )
```

## Data-Driven Operations

Notice that `pick` takes a **value** to determine its behavior. This is **data-driven programming** - the operation's behavior depends on stack contents, not just code.

This pattern appears everywhere in programming:
- Array indexing: `arr[n]`
- Function dispatch: `methods[name]()`
- Configuration: `options[key]`

## Relationship to Other Words

- `0 pick` = `dup`
- `1 pick` = `over`

## The Solution

The stack is `( 100 200 300 )`. You want to copy 100 to the top.
- 300 is at depth 0
- 200 is at depth 1
- 100 is at depth 2

```seq
100 200 300
2 pick
```
