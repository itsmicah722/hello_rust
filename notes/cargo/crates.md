## Crates

A crate is the smallest unit of code that the Rust compiler will consider at a time. Even when
invoking `rustc` directly, it will consider the source file itself as a crate. Crates can be of two
types:

- **Binary**: which means the crate is a standalone executable that is runnable by the operating
  system. It contains a `main()` function entrypoint.
- **Library**: which means the crate does not compile to an executable, but instead contains code
  which can be used by binaries. Library crates do not have `main()` functions.
