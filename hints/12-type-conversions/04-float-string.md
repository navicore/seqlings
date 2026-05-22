# Hint: Float/String Conversions

## string->float

Parses a string as a floating-point number — but watch the return shape: `( String -- Float Bool )`. The Bool is `false` if the string isn't a valid float. For this test the input is a clean `"3.5"`, so the Bool is always `true`; you need to drop it before continuing.

The body is therefore four operations:

1. Push the input string.
2. Call `string->float`. Stack now has `( Float Bool )`.
3. **`drop`** to discard the success flag, leaving just the Float.
4. Add 1.5 with `f.+`.

The dropped-Bool step is the trap; without it, your next operation tries to add a Float to a Bool and the type checker stops you.

## float->string

The reverse direction. Stack effect `( Float -- String )` — no Bool to discard, because *any* Float has a string representation.

```seq
3.14159 float->string    # Stack: ( "3.14159" )
```

The asymmetry (string→float can fail, float→string can't) shows up across every parse/format pair you'll meet: ints, floats, JSON, dates, etc.
