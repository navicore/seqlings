# Types in Seq

Seq is statically typed - the compiler checks types at compile time. Understanding types helps you write correct code.

## Basic Types

- **Int** - Integers: `42`, `-7`, `0`
- **Float** - Floating-point: `3.14`, `-2.5`, `0.0`
- **String** - Text: `"hello"`, `"world"`
- **List** - Ordered collections created via `string.split`
- **Map** - Key-value stores: created with `map.make`
- **Quotation** - Code blocks: `[ dup i.multiply ]`

## Type-Specific Operations

Operations are namespaced by type:

**Integer operations:**
- `i.add`, `i.subtract`, `i.multiply`, `i.divide`, `i.mod`
- `i.negate`, `i.abs`

**Float operations:**
- `f.add`, `f.subtract`, `f.multiply`, `f.divide`
- `f.=`, `f.<`, `f.>`, `f.<=`, `f.>=`

**String operations:**
- `string.concat`, `string.length`, `string.substring`

**List operations:**
- `list.length`, `list.empty?`, `list.map`, `list.filter`, `list.fold`

## Stack Effects Document Types

Stack effect comments show the types:
```seq
: double ( Int -- Int )
    dup i.add
;
```

## Exercises in This Section

1. **predicates** - Understanding type checking
2. **int-ops** - Integer operations
3. **float-ops** - Float operations
4. **string-type** - String operations
5. **list-type** - List operations
