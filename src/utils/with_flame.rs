use super::super::flame;
use std::io::prelude::*;
use std::fs::File;

pub fn perf(name: &'static str) -> flame::SpanGuard {
    flame::start_guard(name)
}

pub fn export() {
    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
