# Variants (Union Types)

Variants let you define types that can be one of several alternatives. This is essential for representing values that could be in different states.

## Defining Variants

```seq
variant Result
    | Ok value
    | Err message
end
```

This defines `Result` as a type that is either `Ok` (with a value) or `Err` (with a message).

## Creating Variant Values

```seq
42 Ok         # Create an Ok variant containing 42
"fail" Err    # Create an Err variant containing "fail"
```

## Matching on Variants

Use pattern matching to handle each case:

```seq
my-result
variant.match
    | Ok -> [ "Got: " swap int->string string.concat ]
    | Err -> [ "Error: " swap string.concat ]
end
```

## Common Patterns

### Option Type
```seq
variant Option
    | Some value
    | None
end
```

### Result Type
```seq
variant Result
    | Ok value
    | Err error
end
```

## Checking Variant Tags

```seq
my-result variant.tag    # Returns "Ok" or "Err"
my-result Ok?            # Returns true if Ok variant
```

## Exercises in This Section

1. **basics** - Creating variant values
2. **option** - Working with Option types
3. **result** - Working with Result types
4. **match** - Pattern matching on variants
5. **combine** - Building with variants
