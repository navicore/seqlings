# Hint: dip

`dip` hides the top value, runs the quotation, then puts it back:
```seq
20 5 [ 10 i.+ ] dip
# 5 is hidden, 20 + 10 = 30, 5 restored → ( 30 5 )
```
