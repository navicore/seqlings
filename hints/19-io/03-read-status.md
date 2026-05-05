# Hint: EOF Handling

`io.read-line` returns `( String Bool )` — `( line true )` on success, `( "" false )` on EOF or I/O error. The Bool feeds straight into `if`, so the standard EOF-handling shape is `io.read-line [ ... ] [ drop ... ] if`.
