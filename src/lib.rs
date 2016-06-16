#![feature(plugin)]
#![plugin(peg_syntax_ext, clippy)]
#![cfg_attr(test, plugin(stainless))]
#![allow(let_unit_value)]

extern crate rand;

#[cfg(feature="flame")]
extern crate flame;

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
mod utils;

use statement::*;
use state::*;
use utils::perf;

const BUILTINS_FILE: &'static str = "builtins.y";
const BUILTINS: &'static str = include_str!("builtins.y");

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
    let _perf = perf("compile");
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
    let _perf = perf("run_program");
    let mut s = State::new();

    {
        let _perf = perf("start_builtins");
        let tokens = grammar::file(BUILTINS);

        match s.run(&tokens.unwrap()) {
            Ok(()) => {}
            Err(e) => e.panic(BUILTINS_FILE),
        }
    }

    let tokens = compile(filename).unwrap();

    match s.run(&tokens) {
        Ok(()) => {}
        Err(e) => e.panic(filename),
    }

    utils::export();
}
