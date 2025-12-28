# Hint: The call Word

`call` takes a quotation and executes it.

## Solution

```seq
7
[ dup i.multiply ] call
```

## Understanding call

`call` is the bridge between "code as data" and "code executing":

```
Before call: ( 7 [dup i.multiply] )
After call:  ( 49 )
```

The quotation consumed 7 and produced 49.

## call is Higher-Order

`call` itself is a higher-order word - it takes a function (quotation) as its argument. This is "meta" - a function operating on functions.
