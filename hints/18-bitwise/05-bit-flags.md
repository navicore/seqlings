# Hint: Bit Flags

The four operations all take ( flags mask -- ... ) and combine them with one bitwise word:

- **set-flag** — OR them together; the masked bit ends up set, others untouched.
- **toggle-flag** — XOR; the masked bit flips, others untouched.
- **clear-flag** — invert the mask first (every bit set EXCEPT the one you want off), then AND with flags.
- **flag-set?** — AND with the mask to isolate just that bit; the result is non-zero exactly when the bit was set.

The body of each helper is one or two words. Once those compile, the test exercises them in sequence and the assertions tell you which one (if any) is off.
