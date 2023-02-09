# [`serde-xml-rs`](https://github.com/RReverser/serde-xml-rs) bug example

A minimal example that reproduces the bug for [issue #197](https://github.com/RReverser/serde-xml-rs/issues/197).

When run, the code shows:
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Writer { source: DocumentStartAlreadyEmitted }', src/main.rs:24:33
```
