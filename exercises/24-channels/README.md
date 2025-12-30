# Part 24: Channels

Channels are Seq's mechanism for communication between concurrent strands (lightweight threads). They provide a safe way to pass data between independent computations.

## What Are Channels?

A channel is a typed conduit for values:
- One strand can **send** values into a channel
- Another strand can **receive** values from a channel
- Channels synchronize the strands automatically

This is based on CSP (Communicating Sequential Processes), the same model used by Go's goroutines and channels.

## Creating Channels

```seq
chan.make   # Create a new channel ( -- Chan )
```

Channels are typed - a channel of Int can only carry Int values.

## Sending and Receiving

```seq
chan value chan.send      # Send value into channel
chan chan.receive         # Receive value from channel
```

Receiving **blocks** until a value is available. This is how channels synchronize concurrent code.

## Safe Operations

```seq
chan value chan.send-safe     # Returns Bool (true if sent)
chan chan.receive-safe        # Returns (value true) or (false)
```

Safe operations don't block forever - they return a status.

## Closing Channels

```seq
chan chan.close   # Signal no more values will be sent
```

After closing:
- Sends fail
- Receives return remaining values, then fail

## Why Channels Matter

Channels solve the fundamental problem of concurrent programming: how do separate computations communicate safely?

Instead of shared mutable state (error-prone), channels provide:
- **Clear ownership**: data moves through channels
- **Synchronization**: built into send/receive
- **Composition**: channels can be combined in patterns

This model makes concurrent code easier to reason about.

## The Connection to Higher-Order Programming

Channels work beautifully with quotations:

```seq
chan.make
dup [ some-work chan.send ] spawn
chan.receive
```

The quotation runs concurrently, communicating back via the channel.

## Concepts You'll Practice

| Concept | What You'll Learn |
|---------|-------------------|
| **CSP (Communicating Sequential Processes)** | Concurrency through message-passing, not shared memory |
| **Channel** | A typed conduit for safe inter-strand communication |
| **Synchronization** | How send/receive coordinate concurrent execution |
| **Producer-Consumer** | A fundamental pattern for concurrent data flow |

CSP is a formal model for concurrent computation developed by Tony Hoare in 1978. The core insight: **"Don't communicate by sharing memory; share memory by communicating."** This eliminates entire classes of concurrency bugs (race conditions, deadlocks from locks).

The final exercise implements the producer-consumer pattern - a building block for pipelines, work queues, and event systems.

*For deeper exploration, see the [Seq Glossary](https://github.com/navicore/patch-seq/blob/main/docs/GLOSSARY.md).*
