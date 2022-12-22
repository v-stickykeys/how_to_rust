This package contains completed exercises from the book [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/).
You can run their tests with cargo:
```sh
cargo test --lib
```

You can also use miri to get information about "undefined behavior" in unsafe code:
```shell
rustup +nightly component add miri
MIRIFLAGS="-Zmiri-tag-raw-pointers" cargo +nightly miri test --lib
```

To silence dead code warnings add this to the start of the test command:
```shell
RUSTFLAGS="$RUSTFLAGS -A dead_code"
```