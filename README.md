# libuwuc

A libc implementation written in Rust.

## Layout

libuwuc consists of two crates, `libuwuc` and `rawc`. `libuwuc` is a normal Rust library
and can be unit tested. It contains all the logic. `rawc` is a wrapper that declares all
the symbols and is compiled to a staticlib/cdylib and then linked into your favourite C programs.

## How to build and test

You can build libuwuc using `cargo build`. Then, use `./uwuc-gcc` to build C programs.

`cargo test -p libuwuc` runs unit tests for the `libuwuc` crate. `./test_c.sh` runs
the bespoke integration test suite with tests written in C.

## Why

yes!!!

## Platform Support

`libuwuc` supports anything that supports the Linux system call ABI but is only tested on actual Linux.
Currently, it only supports x86-64, though that may change in the future.
