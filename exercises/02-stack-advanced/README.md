# Part 2: Advanced Stack Operations

Now that you know the basics, let's explore more powerful stack manipulations. These operations let you access values deeper in the stack without restructuring everything above them.

## Why More Stack Operations?

The basic operations (`dup`, `drop`, `swap`, `over`, `rot`) can express any stack manipulation - but sometimes awkwardly. The advanced operations provide convenience and clarity for common patterns.

## The Advanced Operations

| Word    | Effect                    | Description                        |
|---------|---------------------------|------------------------------------|
| `nip`   | `( a b -- b )`            | Drop the second element            |
| `tuck`  | `( a b -- b a b )`        | Copy top below second              |
| `2dup`  | `( a b -- a b a b )`      | Duplicate top two elements         |
| `3drop` | `( a b c -- )`            | Drop top three elements            |
| `pick`  | `( ...a n -- ...a x )`    | Copy element at depth n            |
| `roll`  | `( ...a n -- ...b )`      | Rotate n elements                  |

## Thinking in Compositions

Advanced programmers realize that `nip` is just `swap drop`:

```seq
( a b )      # Start
( b a )      # After swap
( b )        # After drop
```

And `tuck` is `swap over`:

```seq
( a b )      # Start
( b a )      # After swap
( b a b )    # After over
```

Understanding these equivalences helps you think about stack operations as composable primitives - a key insight for functional programming.

## The Power of pick and roll

`pick` and `roll` work with a depth parameter. This is your first encounter with **data-driven control flow** - the operation's behavior depends on a value on the stack.

```seq
1 2 3 4  2 pick   # Copies the element 2 positions down (the 2)
                  # Stack: ( 1 2 3 4 2 )
```

This pattern - using data to control behavior - is fundamental to programming. You'll see it again with quotations and higher-order functions.
