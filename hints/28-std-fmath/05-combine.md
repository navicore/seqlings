# Hint: Mathematical Formulas

Three small functions, each composing builtins from this chapter.

## circle-area

Area of a circle is `π × r²`. Two operations:

1. Square the radius with the `dup f.*` idiom from the hypotenuse exercise.
2. Multiply by `f.pi`.

## degrees-to-radians

The formula is `degrees × π / 180`. Multiply the input by `f.pi`, then divide by `180.0`. The order matters because Seq doesn't reorder operations for you — you'd get the same final answer as `× (π/180)`, but writing it as `× π ÷ 180` keeps the operands at hand.

## distance

Euclidean distance between `(x1,y1)` and `(x2,y2)` is `sqrt((x2-x1)² + (y2-y1)²)`. The stack going in is `( x1 y1 x2 y2 )` — y2 on top.

Once you have the two differences, the shape mirrors the hypotenuse exercise: square each, add, square root. The interesting bit is GETTING those two differences from a 4-deep stack — each needs the right operands adjacent in the right order for `f.-`.

`rot` brings the third-from-top to the top, which is exactly what's needed to bring `y1` up next to `y2`. After computing the first difference and squaring it, two more `rot`s bring the x-pair to the top for the second difference. Then add the two squared distances and take the root.

## Why `f.abs` in the circle and degrees tests but not distance

The first two tests compare against irrational numbers (`π × 4` and `π` respectively), so floating-point exactness is fiction. They assert "within 0.01" or "within 0.0001" via `f.abs ... f.<`, which is why this is the only exercise in the chapter that needs `include std:fmath` (for `f.abs`).

The `distance` test uses the 3-4-5 triangle, which lands exactly on 5.0 in IEEE 754 — so plain `f.=` works.
