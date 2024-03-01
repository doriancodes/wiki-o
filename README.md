# wiki-o

Smart note taking cli app

## Usage

### Implemented commands

```console
$ wiki-o --help
Create a smart wiki from command line

Usage: wiki-o [COMMAND]

Commands:
  add     Add note
  list    List all notes
  delete  Purge all notes
  init    Initialize wiki-o configuration
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print hel
```

### Initialize configuration

```console
$ wiki-o init
Config initialized
```

### Add note

```console
$ wiki-o add "hello world"
Added hello world to my_notes

$ wiki-o add "another note" -f=new_file
Added another note to new_file
```

### List

```console
$ wiki-o list
File: <home_dir>/wiki-o/.notes/new_file.md

another note


File: <home_dir>/wiki-o/.notes/my_notes.md

hello world
$ wiki-o list -s=true
File: <home_dir>/wiki-o/.notes/new_file.md
File: <home_dir>/wiki-o/.notes/my_notes.md
```

### Delete

```console
$ wiki-o delete
Deleted all notes
```

### Configuration

By default the config file is located under `~/.config/wiki-o`.

```toml
[directories]
notes = "wiki-o/.notes"
config = "wiki-o/.config"

[editor]
format = "md"
```
