# Seqlings Curriculum Plan

A comprehensive curriculum covering ALL Seq built-ins and standard library.

---

## Part 1: Foundations

### 00-intro (3 exercises) ✓ DONE
- 01-hello - io.write-line, basic program structure
- 02-comments - Comment syntax
- 03-numbers - Integer literals, stack effects

### 01-stack-basics (6 exercises) ✓ DONE
- 01-push - Pushing literals
- 02-dup - Duplicate top
- 03-drop - Discard top
- 04-swap - Exchange top two
- 05-over - Copy second to top
- 06-rot - Rotate top three

### 02-stack-advanced (6 exercises)
- 01-nip - Drop second element
- 02-tuck - Copy top below second
- 03-2dup - Duplicate top two
- 04-3drop - Drop three elements
- 05-pick - Copy from depth n
- 06-roll - Rotate n elements

### 03-arithmetic (6 exercises)
- 01-add - i.add
- 02-subtract - i.subtract (operand order!)
- 03-multiply - i.multiply
- 04-divide - i.divide (integer division)
- 05-combine - Chaining operations
- 06-expressions - Complex expressions

### 04-floats (5 exercises)
- 01-float-literals - Float syntax
- 02-f-add - f.add, f.subtract
- 03-f-multiply - f.multiply, f.divide
- 04-f-compare - f.=, f.<, f.>, f.<=, f.>=, f.<>
- 05-mixed - Combining int and float operations

### 05-comparison (5 exercises)
- 01-equals - = operator
- 02-less-greater - < and >
- 03-not-equal - <> operator
- 04-less-equal - <= and >=
- 05-chaining - Multiple comparisons

### 06-boolean (4 exercises)
- 01-and - and operator
- 02-or - or operator
- 03-not - not operator
- 04-combine - Complex boolean expressions

---

## Part 2: Control Flow & Functions

### 07-conditionals (5 exercises)
- 01-if-then - Simple if/then
- 02-if-else - if/else/then
- 03-nested - Nested conditionals
- 04-cond - Multi-way cond
- 05-fizzbuzz - Classic exercise

### 08-words (6 exercises)
- 01-define - Basic : name ... ;
- 02-stack-effects - Writing ( inputs -- outputs )
- 03-calling - Words calling words
- 04-refactor - Extract common patterns
- 05-helper-words - Building vocabulary
- 06-documentation - Self-documenting code

### 09-recursion (5 exercises)
- 01-countdown - Simple recursion
- 02-factorial - Classic factorial
- 03-fibonacci - Fibonacci sequence
- 04-accumulator - Tail-recursive patterns
- 05-mutual - Mutual recursion

### 10-quotations (6 exercises)
- 01-basics - [ ... ] syntax
- 02-call - The call word
- 03-times - Looping with times
- 04-while - while loops
- 05-until - until loops
- 06-higher-order - Quotations as arguments

---

## Part 3: Types & Data

### 11-types (5 exercises)
- 01-type-annotations - Explicit types
- 02-int-type - Int operations
- 03-string-type - String basics
- 04-float-type - Float operations
- 05-type-errors - Understanding errors

### 12-type-conversions (5 exercises)
- 01-int-string - int->string
- 02-string-int - string->int (with status)
- 03-int-float - int->float
- 04-float-int - float->int
- 05-float-string - float->string, string->float

### 13-strings (8 exercises)
- 01-concat - string.concat
- 02-length - string.length, string.byte-length
- 03-char-at - string.char-at, char->string
- 04-substring - string.substring
- 05-find-contains - string.find, string.contains, string.starts-with
- 06-split - string.split
- 07-transform - string.trim, string.chomp, string.to-upper, string.to-lower
- 08-compare - string.empty?, string.equal?

### 14-variants (6 exercises)
- 01-create - variant.make-0/1/2/3
- 02-inspect - variant.tag, variant.field-count
- 03-access - variant.field-at
- 04-modify - variant.append, variant.last, variant.init
- 05-unions - Defining union types
- 06-match - Pattern matching with match

### 15-lists (6 exercises)
- 01-basics - list.length, list.empty?
- 02-map - list.map with quotations
- 03-filter - list.filter
- 04-fold - list.fold (reduce)
- 05-each - list.each
- 06-combine - Chaining list operations

### 16-maps (6 exercises)
- 01-create - map.make
- 02-set-get - map.set, map.get
- 03-safe-get - map.get-safe, map.has?
- 04-remove - map.remove
- 05-iterate - map.keys, map.values
- 06-size - map.size, map.empty?

---

## Part 4: Advanced Features

### 17-row-polymorphism (4 exercises)
- 01-passthrough - Understanding ..a
- 02-generic-words - Stack-polymorphic words
- 03-compose - Composing generic words
- 04-real-world - Practical examples

### 18-bitwise (5 exercises)
- 01-band-bor - band, bor
- 02-bxor-bnot - bxor, bnot
- 03-shifts - shl, shr
- 04-intrinsics - popcount, clz, ctz
- 05-flags - Bit flags pattern

---

## Part 5: I/O & System

### 19-io (5 exercises)
- 01-write-line - io.write-line
- 02-read-line - io.read-line
- 03-read-status - io.read-line+ (with EOF handling)
- 04-read-n - io.read-n
- 05-interactive - Building a prompt

### 20-files (5 exercises)
- 01-exists - file.exists?
- 02-slurp - file.slurp
- 03-slurp-safe - file.slurp-safe (error handling)
- 04-for-each-line - file.for-each-line+
- 05-processing - File processing patterns

### 21-args (3 exercises)
- 01-count - args.count
- 02-at - args.at
- 03-cli-tool - Building a CLI tool

### 22-os (6 exercises)
- 01-getenv - os.getenv
- 02-paths - os.path-join, os.path-parent, os.path-filename
- 03-exists - os.path-exists, os.path-is-file, os.path-is-dir
- 04-dirs - os.home-dir, os.current-dir
- 05-platform - os.name, os.arch
- 06-exit - os.exit

### 23-time (3 exercises)
- 01-now - time.now
- 02-nanos - time.nanos (high precision)
- 03-sleep - time.sleep-ms

---

## Part 6: Concurrency

### 24-channels (6 exercises)
- 01-create - chan.make
- 02-send-receive - chan.send, chan.receive
- 03-safe-ops - chan.send-safe, chan.receive-safe
- 04-close - chan.close
- 05-yield - chan.yield
- 06-patterns - Channel patterns

### 25-spawn (5 exercises)
- 01-basics - spawn a strand
- 02-return-value - Getting strand ID
- 03-communication - Strands with channels
- 04-ping-pong - Coordinated concurrency
- 05-worker-pool - Multiple workers

---

## Part 7: Networking

### 26-tcp (5 exercises)
- 01-listen - tcp.listen
- 02-accept - tcp.accept
- 03-read-write - tcp.read, tcp.write
- 04-close - tcp.close
- 05-echo-server - Complete TCP server

---

## Part 8: Standard Library

### 27-std-imath (4 exercises)
- 01-include - Using include std:imath
- 02-basic - abs, sign, neg
- 03-minmax - min, max, clamp
- 04-advanced - mod, gcd, pow, square

### 28-std-fmath (4 exercises)
- 01-include - Using include std:fmath
- 02-basic - f.abs, f.sign, f.neg
- 03-minmax - f.min, f.max, f.clamp
- 04-advanced - f.square

### 29-std-json (6 exercises)
- 01-parse - Parsing JSON strings
- 02-access - Accessing JSON values
- 03-build - Building JSON objects
- 04-arrays - JSON arrays
- 05-nested - Nested structures
- 06-escape - string.json-escape

### 30-std-yaml (4 exercises)
- 01-parse - Parsing YAML
- 02-access - Accessing values
- 03-build - Building YAML
- 04-complex - Complex documents

### 31-std-result (4 exercises)
- 01-basics - Result pattern
- 02-ok-err - Creating results
- 03-handling - Pattern matching results
- 04-chaining - Result pipelines

### 32-std-http (4 exercises)
- 01-get - HTTP GET requests
- 02-post - HTTP POST requests
- 03-headers - Working with headers
- 04-api-client - Building an API client

---

## Part 9: Intermediate Challenges

### 33-intermediate (10 exercises)
Combining concepts from previous sections:
- 01-word-count - Files + strings + maps
- 02-csv-parser - Files + strings + lists
- 03-calculator - Parsing + recursion + stack
- 04-todo-app - JSON + files + CRUD
- 05-chat-server - TCP + channels + spawn
- 06-http-api - HTTP + JSON + handlers
- 07-config-loader - YAML + os + files
- 08-log-analyzer - Files + strings + maps + time
- 09-pipeline - Channels + spawn + quotations
- 10-game-of-life - Maps + lists + recursion

---

## Summary

| Part | Sections | Exercises | Focus |
|------|----------|-----------|-------|
| 1. Foundations | 00-06 | 35 | Basics, stack, math, booleans |
| 2. Control & Functions | 07-10 | 22 | Conditionals, words, recursion, quotations |
| 3. Types & Data | 11-16 | 36 | Types, strings, variants, lists, maps |
| 4. Advanced | 17-18 | 9 | Row polymorphism, bitwise |
| 5. I/O & System | 19-23 | 22 | I/O, files, args, os, time |
| 6. Concurrency | 24-25 | 11 | Channels, spawn |
| 7. Networking | 26 | 5 | TCP |
| 8. Standard Library | 27-32 | 26 | imath, fmath, json, yaml, result, http |
| 9. Intermediate | 33 | 10 | Combined challenges |
| **Total** | **34** | **176** | |

---

## Coverage Checklist

### Built-ins Covered:
- [x] io.* (write-line, read-line, read-line+, read-n)
- [x] args.* (count, at)
- [x] file.* (slurp, slurp-safe, exists?, for-each-line+)
- [x] Type conversions (int->string, string->int, etc.)
- [x] i.* arithmetic (add, subtract, multiply, divide)
- [x] Comparison (=, <, >, <=, >=, <>)
- [x] Boolean (and, or, not)
- [x] Bitwise (band, bor, bxor, shl, shr, bnot, popcount, clz, ctz)
- [x] Stack (dup, drop, swap, over, rot, nip, tuck, 2dup, 3drop, pick, roll)
- [x] chan.* (make, send, send-safe, receive, receive-safe, close, yield)
- [x] Control (call, cond, times, while, until, spawn)
- [x] tcp.* (listen, accept, read, write, close)
- [x] os.* (getenv, home-dir, current-dir, path-*, exit, name, arch)
- [x] string.* (all operations)
- [x] variant.* (all operations)
- [x] list.* (length, empty?, map, filter, fold, each)
- [x] map.* (all operations)
- [x] f.* arithmetic and comparison
- [x] time.* (now, nanos, sleep-ms)
- [x] test.* (covered implicitly in all exercises)

### Stdlib Covered:
- [x] std:imath
- [x] std:fmath
- [x] std:json
- [x] std:yaml
- [x] std:result
- [x] std:http
- [ ] std:stack-utils (optional, can add if needed)
