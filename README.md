# rustydrive [![Rust](https://github.com/jrcichra/rustydrive/actions/workflows/rust.yml/badge.svg)](https://github.com/jrcichra/rustydrive/actions/workflows/rust.yml)

Reads a drive 10 times, 1 megabyte at a time and stops if there are any differences. Works without caching by using `O_DIRECT` and aligned vectors.
