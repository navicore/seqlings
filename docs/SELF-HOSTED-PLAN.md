# Self-Hosted Seqlings: Feature Plan

The goal is to rewrite seqlings in Seq itself. This serves two purposes:
1. Showcase what Seq can do
2. Drive real requirements for language features

## Current Seqlings Requirements

What the Rust version does:

| Capability | Description |
|------------|-------------|
| Load TOML config | Parse `exercises/info.toml` |
| File I/O | Read exercise files, check for `# I AM NOT DONE` marker |
| Process spawning | Run `seqc lint` and `seqc test` on exercises |
| File watching | Detect when exercise files change |
| Terminal UI | Colored output, progress display |
| Signal handling | Catch Ctrl+C for clean exit |

## What Seq Already Has

| Feature | Status | Notes |
|---------|--------|-------|
| File I/O | Done | `file.slurp`, `file.exists?`, etc. |
| String ops | Done | `string.split`, `string.contains`, etc. |
| Strands | Done | `spawn` for concurrent work |
| Channels | Done | CSP-style communication |
| TCP | Done | Network I/O (not needed here) |
| Basic terminal | Partial | `io.write-line` works, no colors |

## Missing Features

### High Priority

#### 1. Process Spawning / Shell Commands
**Need:** Run external commands like `seqc lint exercises/foo.seq`
**Current:** Not available
**Status:** Confirmed valuable - useful beyond seqlings (calling `git`, `curl`, etc.)
**Proposal:**
```seq
# Simple version - run and wait for exit code
"seqc lint foo.seq" process.run    # ( -- Int ) exit code

# Full version - capture output
"seqc lint foo.seq" process.exec   # ( -- String Int ) stdout, exit code

# Or structured:
"seqc" [ "lint" "foo.seq" ] process.spawn  # ( cmd args -- ProcessHandle )
process.wait      # ( ProcessHandle -- Int ) wait for exit, return code
process.stdout    # ( ProcessHandle -- String ) read stdout
```

#### 2. Better Text Processing
**Need:** Parse unstructured output from external tools, general text manipulation
**Current:** Basic string ops exist but may need enhancement
**Status:** Confirmed valuable - needed for many real-world use cases beyond seqlings
**Gaps to investigate:**
- Regex or pattern matching on strings?
- Line-by-line iteration helpers?
- String builders for efficient concatenation?
- Parsing numbers from mixed text?

#### 3. File System Watching
**Need:** Get notified when files change
**Current:** Not available - would have to poll with `time.sleep-ms` (blocking, anti-Seq)
**Proposal:**
```seq
# Event-based watching (fits Seq's concurrent model)
"exercises" fs.watch    # ( path -- WatchChannel )

# Returns a channel that receives events:
# ( WatchChannel -- WatchEvent )
watch-chan chan.receive
match
    FileChanged { >path } -> handle-change
    FileCreated { >path } -> handle-new
    FileDeleted { >path } -> handle-delete
end
```

This fits naturally with strands - spawn a watcher strand that monitors and sends events.

#### 4. Non-Blocking Timers
**Need:** Timeouts, periodic checks without blocking
**Current:** `time.sleep-ms` blocks the strand
**Proposal:**
```seq
# Timer that sends to a channel after delay
1000 timer.after    # ( ms -- TimerChannel ) fires once after 1s

# Periodic timer
500 timer.every     # ( ms -- TimerChannel ) fires every 500ms

# Usage with select (see below)
timer-chan chan.receive   # receive wakes up when timer fires
```

#### 5. Channel Select / Multiplexing
**Need:** Wait on multiple channels (file events + timer + user input)
**Current:** Can only `chan.receive` on one channel at a time
**Proposal:**
```seq
# Select on multiple channels
[ watch-chan timer-chan stdin-chan ] chan.select
# ( [ channels ] -- value channel-index )

# Enables event loop:
[ watch-chan timer-chan ] chan.select
match
    0 -> handle-file-change    # watch-chan fired
    1 -> handle-timer          # timer fired
end
```

### Medium Priority

#### 5. Terminal Colors / ANSI
**Need:** Colored output for progress, errors, success
**Current:** Not available
**Proposal:**
```seq
# Simple approach - ANSI escape helpers
"text" term.red       # ( String -- String ) wrap in red ANSI
"text" term.green
"text" term.bold
"text" term.dim

# Or raw:
"text" 31 term.color  # ( String Int -- String ) ANSI color code
```

#### 6. Terminal Control
**Need:** Clear screen, cursor positioning
**Current:** Not available
**Proposal:**
```seq
term.clear            # ( -- ) clear screen
term.clear-line       # ( -- ) clear current line
10 5 term.move-to     # ( col row -- ) position cursor
```

#### 7. TOML Parsing
**Need:** Parse `exercises/info.toml`
**Current:** Not available (have JSON and YAML in stdlib)
**Proposal:** Add `std:toml` similar to `std:json`
```seq
include std:toml
file.slurp toml-parse   # ( String -- Variant ) parse TOML to variant tree
```
**Alternative:** Switch to JSON or YAML config format (simpler, already supported)

### Lower Priority

#### 8. Signal Handling
**Need:** Catch Ctrl+C for cleanup
**Current:** Not available
**Proposal:**
```seq
signal.on-interrupt [ cleanup ] signal.handle
# Or channel-based:
signal.interrupt-chan   # ( -- Channel ) receives on Ctrl+C
```

#### 9. Stdin Non-Blocking / Line Editing
**Need:** React to user input without blocking
**Current:** `io.read-line` blocks
**Proposal:** Could use channel-based stdin
```seq
io.stdin-chan    # ( -- Channel ) channel that receives lines
```

## Implementation Strategy

### Phase 1: Minimal Viable
1. Add `process.run` / `process.exec` for running seqc
2. Use polling with `time.sleep-ms` initially (not ideal but works)
3. Skip colors initially (plain output)
4. Use JSON instead of TOML for config

### Phase 2: Proper Event Loop
1. Add `fs.watch` with channel-based events
2. Add `timer.after` / `timer.every`
3. Add `chan.select` for multiplexing
4. Now we have a proper non-blocking event loop

### Phase 3: Polish
1. Add terminal colors
2. Add terminal control (clear, cursor)
3. Add signal handling
4. Consider TOML support

## Design Notes

### Why Channels for Everything?
Seq's concurrency model is CSP-based. Making everything channel-based:
- Unifies the programming model
- Enables `chan.select` for multiplexing
- Fits naturally with strands
- No callbacks or async/await complexity

### Polling vs Events
Polling with `time.sleep-ms`:
- Works today
- Wastes CPU
- Blocking sleep is "anti-Seq" (blocks the strand)

Event-based with `fs.watch`:
- Requires new feature
- Efficient (OS-level notification)
- Fits Seq's concurrent model
- More complex implementation (platform-specific)

### Process Spawning Design Questions
- Should we capture stderr separately?
- Should we support streaming stdout (for long-running processes)?
- Should we support environment variable control?
- Should we support shell expansion or just exec?

## References

- [Rustlings implementation](https://github.com/rust-lang/rustlings) - Current inspiration
- [notify crate](https://docs.rs/notify/latest/notify/) - File watching in Rust (what current seqlings uses)
- [Go fsnotify](https://github.com/fsnotify/fsnotify) - Similar file watching in Go
