#!/usr/bin/env bash

cargo build
./uwuc-gcc hello.c -o target/hello
./target/hello
