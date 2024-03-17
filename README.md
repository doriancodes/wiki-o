![test](https://github.com/doriancodes/wiki-o/actions/workflows/test.yml/badge.svg)
[![License](https://img.shields.io/badge/License-GNU_General_Public_License_v3.0-green)](#license)

# wiki-o

Smart note taking cli app

## Usage

### Implemented commands

```console
$ wo --help
Create a smart wiki from command line

Usage: wo [COMMAND]

Commands:
  add     Add note
  show    Show files with similar name
  list    List all notes
  search  Search notes for similar content
  delete  Delete a note
  purge   Purge all notes and wiki-o configuration for a clean slate
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

## Development

For now run all tests in the same thread:

```console
RUST_TEST_THREADS=1 cargo test
```

## TODO

- [ ] Refactor repo structure
- [ ] Improve test coverage
- [ ] Allow piping commands
- [x] Improve search
- [ ] Allow styling
- [ ] Explore interoperability with other tools (editors like vim/emacs, logseq, ollama)
- [ ] Remote sync via ssh
- [ ] Check how to package for release
- [ ] Check how to add docs
- [ ] Add deamon (maybe), watch file changes
- [ ] Explore tags
- [ ] Explore analytics tools
- [ ] Design copy/paste feature
- [ ] Performance tuning

## License

Released under [GNU General Public License v3.0](/LICENSE) by [@doriancodes](https://github.com/doriancodes).
