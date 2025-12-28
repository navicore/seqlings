# File Operations

Seq provides built-in operations for reading and writing files. These operations use the `file.*` namespace.

## Reading Files

```seq
"path/to/file.txt" file.read    # Returns file contents as String
```

## Writing Files

```seq
"Hello, World!" "output.txt" file.write    # Writes string to file
```

## Checking File Existence

```seq
"myfile.txt" file.exists?    # Returns true/false
```

## File Paths

File paths can be absolute or relative to the current working directory.

```seq
"/absolute/path/file.txt" file.read
"relative/path/file.txt" file.read
```

## Stack Effects

```
file.read ( Path -- String )
file.write ( String Path -- )
file.exists? ( Path -- Bool )
```

## Error Handling

File operations can fail (file not found, permission denied, etc.). Consider using safe patterns:

```seq
"file.txt" file.exists?
[ "file.txt" file.read process-contents ]
[ "File not found" io.write-line ]
if-else
```

## Exercises in This Section

1. **read** - Reading file contents
2. **write** - Writing to files
3. **exists** - Checking if files exist
4. **lines** - Working with line-based content
5. **combine** - Complete file workflows
