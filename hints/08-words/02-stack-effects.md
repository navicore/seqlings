# Hint: Stack Effects

The stack effect `( Int Int -- Int Int )` tells you two things: two inputs, two outputs. To produce TWO results from two inputs, you can't just compute them in sequence — each binary op consumes both its inputs. So the first decision is "how do I keep material around for the second computation?"

That's exactly what `2dup` is for: `( a b -- a b a b )`. With four copies on the stack you can compute one result (consumes two), then bring the surviving two inputs to the top for the second result. Words you might reach for to do that bringing-up: `rot rot`, `-rot` (if your dialect has it), or a couple of `swap`s.

One more thing to check: the test pops the **sum** first, then the **product**. That means the sum needs to be on top of the stack when your word returns. Whichever result you compute first ends up BELOW the second one — plan the order accordingly.

## Why the stack-effect comment matters

`( Int Int -- Int Int )` isn't just documentation — the type checker enforces it. If your body produces three values, you'll get a type error; if one, same. Writing the stack effect first and then matching the body to it is a habit worth building early.
