extern crate schwift;
extern crate bincode;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use bincode::SizeLimit;
use bincode::rustc_serialize::encode;

fn main() {
    let source_file = match env::args().nth(1) {
        Some(s) => s,
        None => panic!("First command-line argument must be a schwift source file."),
    };

    let mut dst_filename = source_file.to_string();
    dst_filename.push_str("c");

    let mut dst_file = File::create(dst_filename).expect("Could not create destination file");

    let ast = schwift::compile(&source_file).unwrap();
    let encoded: Vec<u8> = encode(&ast, SizeLimit::Infinite).unwrap();
    dst_file.write_all(&encoded).expect("Failed to write compiled shwift to file");
}
