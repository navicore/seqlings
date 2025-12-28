# Operating System Operations

Seq provides operations for interacting with the operating system through the `os.*` namespace.

## Environment Variables

```seq
"HOME" os.env-get           # Get environment variable
"MY_VAR" "value" os.env-set # Set environment variable
"PATH" os.env-has?          # Check if variable exists
```

## Current Directory

```seq
os.cwd                      # Get current working directory
"/path/to/dir" os.chdir     # Change directory
```

## Process Information

```seq
os.pid                      # Get process ID
```

## Exercises in This Section

1. **env-get** - Reading environment variables
2. **env-set** - Setting environment variables
3. **cwd** - Working with directories
4. **combine** - OS operation patterns
