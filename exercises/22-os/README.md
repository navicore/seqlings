# Operating System Operations

Seq exposes a small `os.*` namespace for environment, path, and
platform queries. Several of these return a `String Bool` pair —
the value plus a flag that says whether the value was actually
available — and you branch on the flag with `if`, the same way
you do with `file.slurp`.

## Reading environment variables

```seq
"PATH" os.getenv [ ...success: value on top... ]
                 [ drop ...missing path... ]
                 if
```

`os.getenv` is read-only. There is no `os.setenv` — set variables
in the parent shell before launching the program.

## Path operations

`os.path-join` glues two segments together with the host's path
separator. The yes/no predicates each return a single Bool:

```seq
"/etc" "hosts" os.path-join         # → "/etc/hosts"
"/etc/hosts"   os.path-exists       # → Bool
"/etc/hosts"   os.path-is-file      # → Bool
"/etc"         os.path-is-dir       # → Bool
```

`os.path-parent` and `os.path-filename` decompose a path and use
the same `String Bool` shape as `os.getenv`.

## Current and home directories

```seq
os.home-dir       # → ( path status )
os.current-dir    # → ( path status )
```

There is no equivalent of `chdir` — the working directory is set
once when the process starts and stays put.

## Platform info

```seq
os.name    # → "linux", "macos", ...
os.arch    # → "x86_64", "aarch64", ...
```

These never fail, so they return a single `String`.

## Stack effects

| Word | Stack Effect | Notes |
|------|--------------|-------|
| `os.getenv`        | `( name -- value status )` | `value` is "" when status is false |
| `os.home-dir`      | `( -- path status )` | |
| `os.current-dir`   | `( -- path status )` | |
| `os.path-exists`   | `( path -- Bool )` | |
| `os.path-is-file`  | `( path -- Bool )` | |
| `os.path-is-dir`   | `( path -- Bool )` | |
| `os.path-join`     | `( a b -- joined )` | uses host separator |
| `os.path-parent`   | `( path -- parent status )` | false at root |
| `os.path-filename` | `( path -- name status )` | false if no filename |
| `os.name`          | `( -- String )` | |
| `os.arch`          | `( -- String )` | |
| `os.exit`          | `( Int -- )` | never returns |

## Exercises in This Section

1. **01-getenv** — read env vars and branch on the present-flag
2. **02-paths** — `os.path-join` plus the `path-is-*` predicates
3. **03-cwd** — read the current working directory with a fallback
4. **04-combine** — build a path under `os.home-dir`
