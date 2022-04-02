# rustydrive [![Rust](https://github.com/jrcichra/rustydrive/actions/workflows/rust.yml/badge.svg)](https://github.com/jrcichra/rustydrive/actions/workflows/rust.yml)

Reads a drive 10 times, 1 megabyte at a time and stops if there are any differences

# NOTE

Struggling to have linux (actually) read the drive 10 times without caching. `O_DIRECT` gives `Invalid argument` errors no matter what block size I use. Currently I have `posix_fadvise` `POSIX_FADV_DONTNEED` in place. But this isn't guaranteed to read the drive N times.
