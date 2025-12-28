# Part 15: Lists and Higher-Order List Operations

Lists are collections of values. What makes them powerful in Seq is the combination with higher-order functions: `map`, `filter`, and `fold`.

## Creating Lists

```seq
[ 1 2 3 ]           # A list of integers
[ "a" "b" "c" ]     # A list of strings
[ ]                 # An empty list
```

## Basic List Operations

| Word           | Effect                          | Description              |
|----------------|--------------------------------|--------------------------|
| `list.length`  | `( list -- Int )`               | Number of elements       |
| `list.empty?`  | `( list -- Bool )`              | True if no elements      |
| `list.at`      | `( list n -- elem )`            | Get element at index n   |

## Higher-Order List Operations

This is where lists and quotations combine powerfully:

| Word          | Effect                                    | Description                    |
|---------------|-------------------------------------------|--------------------------------|
| `list.map`    | `( list quot -- list )`                   | Transform each element         |
| `list.filter` | `( list quot -- list )`                   | Keep elements matching predicate |
| `list.fold`   | `( list init quot -- result )`            | Reduce list to single value    |
| `list.each`   | `( list quot -- )`                        | Execute for each element       |

## map: Transform Each Element

```seq
[ 1 2 3 ] [ 10 i.multiply ] list.map   # [ 10 20 30 ]
```

The quotation is applied to each element. This is the same pattern as JavaScript's `.map()`, Python's `map()`, etc.

## filter: Select Elements

```seq
[ 1 2 3 4 5 ] [ 2 i.mod 0 = ] list.filter   # [ 2 4 ]
```

The quotation is a predicate (returns true/false). Elements where it returns true are kept.

## fold: Reduce to One Value

```seq
[ 1 2 3 4 ] 0 [ i.add ] list.fold   # 10
```

Starts with initial value (0), applies quotation to combine accumulator with each element.

## The Power of Composition

These operations chain naturally:

```seq
[ 1 2 3 4 5 6 ]
[ 2 i.mod 0 = ] list.filter   # [ 2 4 6 ]
[ dup i.multiply ] list.map   # [ 4 16 36 ]
0 [ i.add ] list.fold         # 56
```

Filter evens, square them, sum the result. Each step is a higher-order operation with a quotation.
