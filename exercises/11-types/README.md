# Types in Seq

Seq is dynamically typed but every value has a type at runtime. Understanding types helps you write correct code and debug effectively.

## Basic Types

- **Int** - Integers: `42`, `-7`, `0`
- **Float** - Floating-point: `3.14`, `-2.5`, `0.0`
- **Bool** - Booleans: `true`, `false`
- **String** - Text: `"hello"`, `"world"`
- **List** - Ordered collections: `[ 1 2 3 ]`
- **Map** - Key-value stores: created with `map.make`
- **Quotation** - Code blocks: `[ dup i.* ]`

## Type Predicates

Seq provides predicates to check types at runtime:

```seq
42 int?        # true
3.14 float?    # true
"hi" string?   # true
true bool?     # true
[ 1 2 ] list?  # true
```

## Type-Specific Operations

Operations are type-specific:
- Integer ops: `i.+`, `i.-`, `i.*`, `i./`
- Float ops: `f.+`, `f.-`, `f.*`, `f./`
- String ops: `string.concat`, `string.length`
- List ops: `list.length`, `list.first`

## Why Explicit Types?

Explicit type operations prevent accidental type mixing:
```seq
5 3.0 f.+    # Error! Can't add Int to Float directly
5 int->float 3.0 f.+    # Correct: convert first
```

## Exercises in This Section

1. **predicates** - Using type predicates
2. **int-ops** - Integer-specific operations
3. **float-ops** - Float-specific operations
4. **string-type** - String operations
5. **list-type** - List type checking
