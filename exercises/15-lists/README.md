# Part 15: Lists and Higher-Order List Operations

Lists are collections of values. What makes them powerful in Seq is the combination with higher-order functions: `map`, `filter`, and `fold`.

## Creating Lists

In Seq, lists are created using `string.split`. Lists are actually variants (tagged unions) under the hood:

```seq
"apple banana cherry" " " string.split   # List of 3 strings
"a,b,c" "," string.split                 # Split by comma
```

## Basic List Operations

| Word              | Effect                          | Description              |
|-------------------|--------------------------------|--------------------------|
| `list.length`     | `( Variant -- Int )`           | Number of elements       |
| `list.empty?`     | `( Variant -- Int )`           | 1 if empty, 0 otherwise  |
| `variant.field-at`| `( Variant Int -- value )`     | Get element at index     |

## Higher-Order List Operations

This is where lists and quotations combine powerfully:

| Word          | Effect                                      | Description                    |
|---------------|---------------------------------------------|--------------------------------|
| `list.map`    | `( Variant Quotation -- Variant )`          | Transform each element         |
| `list.filter` | `( Variant Quotation -- Variant )`          | Keep elements matching predicate |
| `list.fold`   | `( Variant init Quotation -- result )`      | Reduce list to single value    |
| `list.each`   | `( Variant Quotation -- )`                  | Execute for each element       |

## map: Transform Each Element

```seq
"apple banana" " " string.split [ string.to-upper ] list.map
# Result: list of "APPLE" "BANANA"
```

The quotation is applied to each element. This is the same pattern as JavaScript's `.map()`, Python's `map()`, etc.

## filter: Select Elements

```seq
"apple banana cherry" " " string.split [ string.length 5 > ] list.filter
# Result: list of "banana" "cherry" (length > 5)
```

The quotation is a predicate (returns 0 or non-zero). Elements where it returns non-zero are kept.

## fold: Reduce to One Value

```seq
"apple banana cherry" " " string.split "" [ string.concat ] list.fold
# Result: "applebananacherry"
```

Starts with initial value (""), applies quotation to combine accumulator with each element.

## The Power of Composition

These operations chain naturally:

```seq
"a be see deep" " " string.split
[ string.length 2 > ] list.filter    # keeps "see" "deep"
0 [ string.length i.add ] list.fold  # 7 (3 + 4)
```

Filter by length, then sum lengths. Each step is a higher-order operation with a quotation.
