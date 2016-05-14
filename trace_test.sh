#!/bin/sh
cargo test 2> /dev/null | ./parse_trace.py
