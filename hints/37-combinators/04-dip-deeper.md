# Hint: Nested dip

Nest dip inside dip to reach deeper:
```seq
100 5 3
[ [ 50 i.+ ] dip ] dip
# Outer dip hides 3, inner dip hides 5, adds 50 to 100
# Result: ( 150 5 3 )
```
