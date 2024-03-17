![test](https://github.com/doriancodes/wiki-o/actions/workflows/test.yml/badge.svg)
[![License](https://img.shields.io/badge/License-GNU_General_Public_License_v3.0-green)](#license)

# wiki-o

Smart note taking cli app

## Usage

### Implemented commands

```console
$ wo --help
Usage: wo [COMMAND]

Commands:
  add     Add note
  show    Show files with similar name
  list    List all notes
  search  Search notes for similar content
  delete  Delete a note
  purge   Purge all notes and wiki-o configuration for a clean slate
  pa      Piped add note
  config  Show wiki-o configuration
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Show configuration

```console
$ wo config

Current configuration:

notes directory: wiki-o/notes
metadata directory: wiki-o/_metadata
file format: md
```

### Add note

```console
$ wo add "hello world"
Added hello world to my_notes

$ wo add "another note" -f=new_file
Added another note to new_file
```

### List

```console
$ wo list
File: <home_dir>/wiki-o/.notes/new_file.md

another note


File: <home_dir>/wiki-o/.notes/my_notes.md

hello world
$ wo list -s=true
File: <home_dir>/wiki-o/.notes/new_file.md
File: <home_dir>/wiki-o/.notes/my_notes.md
```

### Delete

```console
$ wo delete -f=my_notes
Deleted file: <home_dir>/wiki-o/notes/my_notes.md
```

### Purge

```console
$ wo purge
Deleted directory: <home_dir>/wiki-o/notes
```

### Configuration

By default the config file is located under `~/.config/wiki-o`.

```toml
# config.toml
notes_dir = "wiki-o/notes"
metadata_dir = "wiki-o/_metadata"
file_format = "md
```

## Piping commands

Instead of adding the command through the [add](#add-note), you can pipe the output that you get from another command directly into wiki-o and save it in a note:

```console
$ echo 'hello world' | wo pa

added hello world to my_notes

$ echo 'hello world' | wo pa -f=new_file

added hello world to new_file
```

## Development

For now run all tests in the same thread:

```console
RUST_TEST_THREADS=1 cargo test
```

## Roadmap

Upcoming features/bug fixes can be found [here](/roadmap/TODO.md).

## License

Released under [GNU General Public License v3.0](/LICENSE) by [@doriancodes](https://github.com/doriancodes).
