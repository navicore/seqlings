# Hint: Weave Cancellation

Weaves must be cleaned up! Either resume until done, or cancel explicitly.

## Solution

Just replace `drop` with `strand.weave-cancel`:

```seq
: test-cancellation ( -- )
    [ counter ] strand.weave

    0 strand.resume
    test.assert
    0 test.assert-eq

    0 strand.resume
    test.assert
    1 test.assert-eq

    0 strand.resume
    test.assert
    2 test.assert-eq

    strand.weave-cancel    # <-- Replace 'drop' with this!

    true test.assert
;
```

## When to Use Cancel vs Resume-to-Completion

**Use `strand.weave-cancel` when:**
- The generator is infinite (like our counter/accumulator)
- You want to stop early after getting enough values
- You're handling an error and need to abort

**Resume until `has_more = false` when:**
- The generator has a natural end point
- You need all values
- The generator does cleanup in its final code

## What Happens If You Don't Clean Up?

If you drop a WeaveHandle without cancelling or completing:
- The spawned coroutine hangs forever waiting for resume
- Resources leak
- Your program may not exit properly

## Cancel Is Safe Anywhere

```seq
[ gen ] strand.weave
strand.weave-cancel          # Before first resume - OK!

[ gen ] strand.weave
0 strand.resume drop drop
strand.weave-cancel          # After some yields - OK!

[ gen ] strand.weave
# resume until done...
strand.weave-cancel          # After completion - OK! (no-op)
```
