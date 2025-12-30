# Part 29: Weave - Generators from Strands

Weave creates generators (coroutines) with bidirectional yield/resume semantics. Unlike regular strands that fire-and-forget, weaves allow structured back-and-forth communication.

## What Is Weave?

A weave is a strand that can:
- **Yield** values back to the caller
- **Receive** values when resumed
- Repeat this cycle until done or cancelled

Think of it as a pausable computation that exchanges data at each pause point.

## Core Operations

```seq
[ quotation ] strand.weave    # ( Quotation -- WeaveHandle )
value strand.resume           # ( WeaveHandle a -- WeaveHandle a Bool )
value yield                   # ( WeaveCtx a -- WeaveCtx a )
strand.weave-cancel           # ( WeaveHandle -- )
```

## Basic Flow

```seq
# Create a weave - quotation receives (WeaveCtx, first_resume_value)
[ swap yield swap yield drop ] strand.weave

# Resume with a value, get back yielded value + has_more flag
10 strand.resume    # Stack: ( Handle 10 true )
drop                # Drop the yielded value
20 strand.resume    # Stack: ( Handle 20 true )
drop
0 strand.resume     # Stack: ( Handle 0 false ) - weave completed
drop drop           # Cleanup
```

## The WeaveCtx

Inside a weave, the `WeaveCtx` must be explicitly threaded through `yield`:

```seq
: my-generator ( Ctx Int -- | Yield Int )
    tuck        # ( Int Ctx Int )
    yield       # Send Int, receive next Int
    rot         # Rearrange stack
    i.+         # Process
    my-generator   # Tail recurse
;
```

**Important**: The context travels on the stack. You must pass it to each `yield` call.

## Resource Management

Weaves must be properly cleaned up:

1. **Resume until completion**: When `strand.resume` returns `false`, the weave is done
2. **Explicit cancellation**: Call `strand.weave-cancel` to stop early

```seq
# Option 1: Run to completion
[ ... ] strand.weave
0 strand.resume
if ... 0 strand.resume else drop drop then

# Option 2: Cancel early
[ ... ] strand.weave
0 strand.resume
if drop strand.weave-cancel else drop drop then
```

## Why Weave?

- **Generators**: Produce sequences of values lazily
- **Coroutines**: Interleave computations with the caller
- **State machines**: Maintain state across yield points
- **Resource efficiency**: Only compute values when needed
