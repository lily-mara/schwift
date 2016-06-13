#!/bin/sh
cargo test --features peg/trace 2> /dev/null | ./parse_trace.py
