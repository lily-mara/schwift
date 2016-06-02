#![feature(plugin)]
#![plugin(peg_syntax_ext, clippy)]
#![cfg_attr(test, plugin(stainless))]

extern crate rand;

use std::fs::File;
use std::io::prelude::*;

peg_file! grammar("schwift.rustpeg");

#[cfg(test)]
mod grammar_tests;

pub mod statement;
pub mod expression;
pub mod value;
pub mod error;
pub mod state;

use statement::*;
use state::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equality,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    ShiftLeft,
    ShiftRight,
    And,
    Or,
}

pub fn compile(filename: &str) -> Result<Vec<Statement>, grammar::ParseError> {
    let mut f = match File::open(filename) {
        Result::Ok(i) => i,
        Result::Err(_) => panic!("Failed to open file {}", filename),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Result::Ok(_) => {}
        Result::Err(_) => panic!("Failed to read file {}", filename),
    };
    grammar::file(&s)
}

pub fn run_program(filename: &str) {
    let mut s = State::new();
    let tokens = compile(filename);

    match s.run(&tokens.unwrap()) {
        Ok(()) => {}
        Err(e) => e.panic(filename),
    }
}
