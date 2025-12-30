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
6. **row-polymorphism** - Generic functions with type variables

## Concepts You'll Practice

| Concept | What You'll Learn |
|---------|-------------------|
| **Type Variables** | Using `A`, `B` to write generic functions |
| **Row Polymorphism** | Functions that work with any stack depth |
| **Parametric Polymorphism** | One definition, many types |
| **Type Inference** | How the compiler figures out concrete types |

Row polymorphism is what makes stack-based languages powerful. When you write `( A -- A A )`, the function works for ANY type - Int, String, custom types, anything. The compiler ensures type safety while giving you flexibility.

The final exercise has you implement `keep`, a combinator that demonstrates how one definition serves infinite type combinations.

*For deeper exploration, see the [Seq Glossary](https://github.com/navicore/patch-seq/blob/main/docs/GLOSSARY.md).*
