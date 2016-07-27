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

fn get_line<'a>(file: &'a str, err: &grammar::ParseError) -> &'a str {
    let mut count = 0usize;
    let mut last_newline = 0usize;

    for i in 0..file.len() {
        if file.is_char_boundary(i) {
            let symbol = unsafe { file.slice_unchecked(i, i + 1) };

            if symbol == "\n" {
                count += 1;

                if count == err.line {
                    unsafe {
                        return file.slice_unchecked(last_newline, i);
                    }
                }

                last_newline = i + 1;
            }
        }
    }

    panic!("Got grammar error with invalid line number {}", err.line);
}

fn place_carat(err: &grammar::ParseError) -> String {
    let mut s = String::new();

    for _ in 0..err.column - 1 {
        s.push(' ');
    }

    s.push('^');

    s
}

pub fn compile(filename: &str) -> Vec<Statement> {
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

    match grammar::file(&s) {
        Ok(statements) => statements,
        Err(ref e) => {
            println!("SYNTAX ERROR: {}:{}\n{}\n{}",
                     filename,
                     e.line,
                     get_line(&s, e),
                     place_carat(e));
            std::process::exit(1);

        }
    }
}

pub fn run_program(filename: &str, args: &[&str]) {
    let _perf = perf("run_program");
    let mut s = State::new();

    s.parse_args(&args);

    {
        let _perf = perf("start_builtins");
        let tokens = grammar::file(BUILTINS).unwrap();

        match s.run(&tokens) {
            Ok(()) => {}
            Err(e) => e.panic(BUILTINS_FILE),
        }
    }

    let tokens = compile(filename);

    match s.run(&tokens) {
        Ok(()) => {}
        Err(e) => e.panic(filename),
    }

    utils::export();
}
