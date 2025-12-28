# Seqlings

Interactive exercises for learning [Seq](https://github.com/navicore/patch-seq), a stack-based programming language.

Inspired by [Rustlings](https://github.com/rust-lang/rustlings), Seqlings guides you through Seq's concepts with hands-on exercises that you complete in your editor.

## Prerequisites

You need the Seq compiler (`seqc`) installed and available in your PATH:

```bash
# Clone and build patch-seq
git clone https://github.com/navicore/patch-seq
cd patch-seq
cargo build --release

# Add to PATH (add to your shell profile for persistence)
export PATH="$PATH:$(pwd)/target/release"

# Verify installation
seqc --version
```

## Quick Start

```bash
# Clone seqlings
git clone https://github.com/navicore/seqlings
cd seqlings

# Build and run
cargo run
```

This starts **watch mode** - Seqlings monitors your exercise files and provides instant feedback as you edit them.

## How It Works

1. Seqlings shows you the current exercise
2. Open the exercise file in your editor
3. Read the instructions and fix/complete the code
4. Delete the `# I AM NOT DONE` marker when finished
5. Seqlings automatically verifies and advances to the next exercise

## Commands

| Command | Description |
|---------|-------------|
| `seqlings` | Start watch mode (default) |
| `seqlings list` | Show all exercises with completion status |
| `seqlings hint` | Get a hint for the current exercise |
| `seqlings hint <name>` | Get a hint for a specific exercise |
| `seqlings verify` | Check all exercises at once |
| `seqlings next` | Skip to the next exercise |
| `seqlings reset` | Reset current exercise to original state |
| `seqlings reset <name>` | Reset a specific exercise |

## Curriculum

The exercises cover Seq from basics to advanced topics:

| Section | Topics |
|---------|--------|
| **00-intro** | Hello world, comments, numbers |
| **01-stack-basics** | push, dup, drop, swap, over, rot |
| **02-stack-advanced** | nip, tuck, 2dup, pick, roll |
| **03-arithmetic** | i.add, i.subtract, i.multiply, i.divide |
| **04-floats** | Float literals and f.* operations |
| **05-comparison** | =, <, >, <=, >=, <> |
| **06-boolean** | and, or, not |
| **07-conditionals** | if/then/else |
| **08-words** | Defining words (functions) |
| **09-recursion** | Recursive patterns, TCO |
| **10-quotations** | Higher-order programming with [ ] |
| **11-types** | Type system basics |
| **12-type-conversions** | int->string, string->int, etc. |
| **13-strings** | String operations |
| **14-variants** | Union types and pattern matching |
| **15-lists** | list.map, list.filter, list.fold |
| **16-maps** | Key-value dictionaries |
| **19-io** | Console I/O |
| **20-files** | File operations |
| **21-args** | Command-line arguments |
| **22-os** | Environment and system info |
| **23-time** | Timestamps and timing |
| **24-channels** | CSP-style concurrency |
| **25-spawn** | Green threads (strands) |
| **26-tcp** | Network programming |
| **27-std-imath** | Integer math stdlib |
| **28-std-fmath** | Float math stdlib |

## Seq Language Basics

Seq is a **stack-based**, **concatenative** language. Values go on a stack, and words (functions) consume and produce stack values.

```seq
# Push values onto the stack
10 20

# i.add pops two values, pushes their sum
i.add          # Stack: ( 30 )

# Define a word (function)
: square ( Int -- Int )
    dup i.multiply
;

# Use it
5 square       # Stack: ( 25 )
```

### Key Concepts

**Stack manipulation:**
```seq
dup      # ( a -- a a )       Duplicate top
drop     # ( a -- )           Discard top
swap     # ( a b -- b a )     Exchange top two
over     # ( a b -- a b a )   Copy second to top
rot      # ( a b c -- b c a ) Rotate third to top
```

**Quotations** are deferred code blocks:
```seq
[ dup i.multiply ]     # A quotation (not executed yet)
5 [ dup i.multiply ] call   # Execute it: result is 25
```

**Lists** are created from strings:
```seq
"apple banana cherry" " " string.split   # Creates a 3-element list
[ string.to-upper ] list.map             # Transform each element
```

**Unions** define algebraic data types:
```seq
union Option { Some { value: Int }, None }

42 Make-Some      # Create a Some variant
Make-None         # Create a None variant

# Pattern match
my-option match
    Some { >value } -> value
    None -> 0
end
```

## Tips

- **Read the comments** - Each exercise explains the concept
- **Use `seqlings hint`** - When stuck, get a hint
- **Check solutions/** - Reference solutions are available
- **Experiment** - Try variations in the REPL (`seqc repl`)

## Contributing

Found a bug or want to improve an exercise? PRs welcome!

If you find gaps in the Seq language itself, please open issues at [patch-seq](https://github.com/navicore/patch-seq).

## License

MIT
