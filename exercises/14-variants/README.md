# Unions (Algebraic Data Types)

Unions let you define types that can be one of several alternatives. This is essential for representing values that could be in different states.

## Defining Unions

```seq
union IntResult { Ok { value: Int }, Err { error: String } }
```

This defines `IntResult` as a type that is either `Ok` (with an Int value) or `Err` (with a String error).

## Auto-Generated Constructors

The compiler generates constructor words for each variant:

```seq
42 Make-Ok           # Create an Ok variant containing 42
"fail" Make-Err      # Create an Err variant containing "fail"
```

## Matching on Variants

Use `match/end` to handle each case:

```seq
my-result
match
    Ok { >value } -> value int->string "Got: " swap string.concat
    Err { >error } -> "Error: " swap string.concat
end
```

When matching, you can extract fields using `{ >fieldname }` syntax.

## Common Patterns

### Option Type
```seq
union Option { Some { value: Int }, None }
```

### Result Type
```seq
union IntResult { Ok { value: Int }, Err { error: String } }
```

## Checking Variant Tags

```seq
my-result variant.tag    # Returns tag number (0 for first variant, 1 for second, etc.)

# Define helper predicates:
: is-ok? ( IntResult -- Int )  variant.tag 0 = ;
: is-err? ( IntResult -- Int ) variant.tag 1 = ;
```

## Exercises in This Section

1. **basics** - Creating variant values with constructors
2. **option** - Working with Option types
3. **result** - Working with Result types
4. **match** - Pattern matching on variants
5. **combine** - Building with variants

## Concepts You'll Practice

| Concept | What You'll Learn |
|---------|-------------------|
| **Algebraic Data Type (ADT)** | Defining types as combinations of alternatives |
| **Sum Type** | A type that can be one of several variants ("or") |
| **Pattern Matching** | Branching based on data structure, not just values |
| **Exhaustiveness** | Compiler ensures you handle all cases |

ADTs are one of the most powerful ideas in programming language design. Languages like Haskell, Rust, and OCaml are built around them. The `Option` and `Result` types you'll implement here are the foundation of safe error handling - no null pointer exceptions!

The final exercise shows how `Result` types make error handling explicit and composable.

*For deeper exploration, see the [Seq Glossary](https://github.com/navicore/patch-seq/blob/main/docs/GLOSSARY.md).*
