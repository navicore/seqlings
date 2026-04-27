# Part 18: Bitwise Operations

Bitwise operations work on the binary representation of an integer, one bit at a time. They're how compilers turn arithmetic into single fast instructions, and how systems APIs pack many on/off settings into a single integer.

## The Operators

| Word    | Stack effect       | What it does                                        |
|---------|--------------------|-----------------------------------------------------|
| `band`  | `( Int Int -- Int )` | bit set in BOTH inputs → 1                          |
| `bor`   | `( Int Int -- Int )` | bit set in EITHER input → 1                         |
| `bxor`  | `( Int Int -- Int )` | bit set in EXACTLY ONE input → 1                    |
| `bnot`  | `( Int -- Int )`     | flip every bit                                      |
| `shl`   | `( Int Int -- Int )` | shift left by N bits (≡ multiply by 2^N)            |
| `shr`   | `( Int Int -- Int )` | shift right by N bits (≡ integer divide by 2^N)     |

## A Mental Model

Think of an integer as a row of bits. The bitwise operators apply the same logical rule to every column independently:

```
  12 = 1100
  10 = 1010
  ----
   AND  1000 = 8
   OR   1110 = 14
   XOR  0110 = 6
```

`bnot` is single-input — it just flips every bit. In two's complement (the representation Seq uses), `bnot 0 = -1` because all-bits-set is the integer -1.

Shifts move bits sideways. Left-shift fills the new low bits with 0; right-shift fills the new high bits with 0.

## Why Bother?

- **Masks**: `value band mask` keeps only the bits you care about — useful for parsing protocol headers, color channels, file modes.
- **Flags**: pack many on/off settings into one integer. Set with `bor`, clear with `bnot band`, test with `band` (chapter exercise 5).
- **Fast multiplication**: `shl` is one CPU cycle; multiplying by a power of 2 is the same operation.
- **Fast hashing**: most hash functions are XOR + shift + multiply.

## Concepts You'll Practice

| Concept            | What You'll Learn                                |
|--------------------|--------------------------------------------------|
| **Bit-level AND/OR** | Masking, combining flags                         |
| **XOR**            | Differences and self-inverse property            |
| **NOT**            | Two's complement representation                  |
| **Shifts**         | The multiplication/division equivalence          |
| **Bit flags**      | The canonical real-world packing pattern         |

For more advanced bit-twiddling — branchless code, popcount tricks, log2 — see the [hacker's delight examples in patch-seq](https://github.com/navicore/patch-seq/tree/main/examples/projects/hackers-delight).
