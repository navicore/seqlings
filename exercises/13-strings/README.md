# Part 13: Strings

Strings are sequences of characters. Seq provides comprehensive string operations for text processing.

## String Literals

```seq
"Hello, World!"     # A string
""                  # Empty string
"Line 1\nLine 2"    # With escape sequences
```

## Core String Operations

| Word              | Effect                           | Description                |
|-------------------|----------------------------------|----------------------------|
| `string.length`   | `( str -- Int )`                 | Number of characters       |
| `string.concat`   | `( str str -- str )`             | Join two strings           |
| `string.char-at`  | `( str n -- char )`              | Get character at index     |
| `string.substring`| `( str start len -- str )`       | Extract portion            |

## Searching

| Word               | Effect                    | Description                    |
|--------------------|---------------------------|--------------------------------|
| `string.find`      | `( str sub -- Int )`      | Index of substring (-1 if not) |
| `string.contains`  | `( str sub -- Bool )`     | Does it contain substring?     |
| `string.starts-with`| `( str pre -- Bool )`    | Does it start with prefix?     |

## Transformation

| Word              | Effect                    | Description              |
|-------------------|---------------------------|--------------------------|
| `string.to-upper` | `( str -- str )`          | Convert to uppercase     |
| `string.to-lower` | `( str -- str )`          | Convert to lowercase     |
| `string.trim`     | `( str -- str )`          | Remove leading/trailing whitespace |
| `string.split`    | `( str delim -- list )`   | Split into list of strings |

## Comparison

| Word              | Effect                    | Description              |
|-------------------|---------------------------|--------------------------|
| `string.equal?`   | `( str str -- Bool )`     | Are strings equal?       |
| `string.empty?`   | `( str -- Bool )`         | Is string empty?         |

## Strings and Higher-Order Operations

Strings work with list operations when split:

```seq
"a,b,c" "," string.split              # [ "a" "b" "c" ]
[ string.to-upper ] list.map          # [ "A" "B" "C" ]
"," [ string.concat ] list.fold       # "A,B,C" (almost - needs work)
```
