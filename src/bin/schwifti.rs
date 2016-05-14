extern crate schwift;
extern crate bincode;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use bincode::rustc_serialize::decode;

fn main() {
    let ast_filename = match env::args().nth(1) {
        Some(s) => s,
        None => panic!("First command-line argument must be a compiled schwift file."),
    };

    let mut ast_file = File::open(ast_filename).expect("Could not open schwift file");
    let mut buf = Vec::new();
    ast_file.read_to_end(&mut buf).expect("Failed to read from compiled shwift file");

    let ast: Vec<schwift::Statement> = decode(&buf).unwrap();

    let mut state = schwift::State::new();
    state.run(ast);
}
