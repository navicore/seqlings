# Part 7: Conditionals

Conditionals let programs make decisions. With comparisons and booleans under your belt, you can now control program flow.

## The if/then Form

```seq
condition if
    # code to run if true
then
```

The `if` consumes a boolean from the stack. If true, the code before `then` runs. If false, it's skipped.

**Important for if/then (without else)**: The body must have a *balanced* stack effect - it should leave the stack the same whether it runs or not. Common pattern: push a default value, then conditionally replace it.

```seq
0              # default value
5 3 i.> if
    drop 42    # replace default with 42
then
# Stack: ( 42 ) because condition was true
```

## The if/else/then Form

```seq
condition if
    # code if true
else
    # code if false
then
```

Exactly one branch executes, never both. This form is easier because both branches run, so they just need to match each other.

## Stack Effects in Branches

**Important**: Both branches must have the same stack effect!

```seq
# WRONG - branches have different effects
x 0 i.> if
    42      # Pushes one value
else
    1 2     # Pushes two values!
then
```

The type checker catches this error. This ensures your code is predictable.

## Conditionals as Expressions

Unlike many languages, Seq conditionals can return values:

```seq
: abs ( Int -- Int )
    dup 0 i.< if
        -1 i.*   # Negate if negative
    then
;
```

This works because the body (`-1 i.*`) has a balanced effect: it replaces one value with another. Whether the body runs or not, you end up with one Int on the stack.
