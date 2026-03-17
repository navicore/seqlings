# Hint: Calculate Speedup

Maximum speedup = total_time / serial_time:
```seq
40 10 i./
```

With 25% serial work, you can never go faster than 4x, even with infinite workers. This is the fundamental limit Amdahl's Law reveals.
