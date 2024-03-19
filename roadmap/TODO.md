# TODO

## Next release

- [x] Refactor repo structure
- [ ] Refactoring: file format only in config, otherwise merged with filename (given by user with filename)
- [ ] Improve error messages, maybe using [human-panic](https://crates.io/crates/human-panic) and mappying to specific text [`with_context`](https://rust-cli.github.io/book/tutorial/errors.html)
- [ ] Testing
  - [ ] Improve coverage (~70/80%), add github badge
  - [x] Run test in parallel
  - [ ] Test optional file names and file format all cases
- [x] Allow piping commands
- [x] Prepare for release
  - [x] Generate `CHANGELOG.md`
  - [x] Check how to package for release
  - [x] Check how to add docs
- [x] Improve project repo
  - [x] add renovate for automatic dependency updates
  - [x] add trivy for security checks
  - [x] add linting and code smells detection in workflow

## Future releases

- [ ] Add features
  - [ ] Remote sync via ssh
  - [ ] Add deamon (maybe), watch file changes
  - [ ] Design copy/paste feature
- [ ] Improve search:
  - [ ] merging indexes on existing files
  - [ ] deleting an index when a file is deleted
  - [ ] Inject search engine in actions
- [ ] Misc
  - [ ] Explore clipboard capabilities
  - [ ] Explore tags
  - [ ] Explore interoperability with other tools (editors like vim/emacs, logseq, ollama)
  - [ ] Explore analytics tools
  - [ ] Performance tuning
- [ ] Logging
  - [ ] Generic log functions
  - [ ] replace `println!` with `write!` (better performance)
  - [ ] Allow styling in config
  - [ ] Separate styling configuration from execution