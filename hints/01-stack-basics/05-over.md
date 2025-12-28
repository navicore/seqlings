# Hint: over

The `over` word copies the second element to the top.

## How over Works

```seq
1 2 over    # Stack: ( 1 2 1 )
```

Stack effect: `( a b -- a b a )`

The second element is copied (not moved) to the top.

## The Solution

Starting with `( 5 10 )`, you need `( 5 10 5 )`. Add `over`:

```seq
5 10
over
5 test.assert-eq
10 test.assert-eq
5 test.assert-eq
```

## over vs dup

- `dup` copies the top: `( a -- a a )`
- `over` copies the second: `( a b -- a b a )`

## When to Use over

`over` is useful when you need the second value but also want to keep both:

```seq
: add-and-keep-both ( Int Int -- Int Int Int )
    over over i.+
;
# 3 5 add-and-keep-both leaves ( 3 5 8 )
```
