# Part 25: Spawning Strands

Strands are Seq's lightweight concurrent threads. Combined with channels, they enable powerful concurrent patterns.

## What Are Strands?

A strand is an independent computation that runs concurrently with others. Unlike OS threads:
- Strands are lightweight (thousands can run efficiently)
- They're cooperatively scheduled
- They communicate via channels, not shared memory

## Spawning a Strand

```seq
[ quotation ] spawn   # Returns strand ID
```

The quotation runs in a new strand. The current strand continues immediately.

## Communicating via Channels

Strands typically communicate through channels:

```seq
chan.make
dup [
    # Worker strand
    heavy-computation
    chan.send
] spawn
drop   # Don't need strand ID
chan.receive   # Wait for result
```

## Strand Patterns

**Worker**: Spawn, do work, send result back
```seq
chan.make dup [ work chan.send ] spawn drop chan.receive
```

**Pipeline**: Chain of workers, each processing and passing on
```seq
chan1 chan2
[ chan1.receive process chan2.send ] spawn
```

**Fan-out**: One producer, multiple workers
```seq
[ chan.receive process ] spawn
[ chan.receive process ] spawn
# Both workers receive from same channel
```

## Why Strands + Channels?

This model (from CSP - Communicating Sequential Processes) makes concurrent programming manageable:

1. **No shared mutable state** - data flows through channels
2. **Explicit communication** - you see where data moves
3. **Composable** - patterns combine cleanly

The same ideas power Go's goroutines, Erlang's processes, and many other successful concurrent systems.
