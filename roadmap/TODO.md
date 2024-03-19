# TODO

## Next release

- [x] Refactor repo structure
- [ ] Refactoring: file format only in config, otherwise merged with filename (given by user with filename)
- [ ] Testing
  - [ ] Improve coverage (~70/80%), add github badge
  - [x] Run test in parallel
  - [ ] Test optional file names and file format all cases
- [x] Allow piping commands
- [ ] Prepare for release
  - [ ] Generate `CHANGELOG.md`
  - [ ] Check how to package for release
  - [ ] Check how to add docs
- [ ] Improve project repo
  - [ ] add dependabot or renovate for automatic dependency updates
  - [ ] add trivy for security checks
  - [ ] add linting and code smells detection in workflow

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