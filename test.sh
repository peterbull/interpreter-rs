#!/bin/bash

cargo build
RUST_BACKTRACE=1 ./target/debug/interpreter-rs tokenize ./lox/hello.lox
