# Development

## Remove code smells

This project uses clippy.

```console
$ cargo clippy
```

## Remove unused dependencies

To optimize packaging for release [`cargo-udepts`](https://github.com/est31/cargo-udeps) is used.

```console
$ cargo install cargo-udeps --locked
$ cargo +nightly udeps
```
