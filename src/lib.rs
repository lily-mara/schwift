use std::{fs::File, io::prelude::*};

mod grammar;

#[cfg(test)]
mod grammar_tests;

pub mod error;
pub mod expression;
pub mod state;
pub mod statement;
mod util;
pub mod value;
mod vec_map;

use crate::{state::*, statement::*};

const BUILTINS_FILE: &str = "builtins.y";
const BUILTINS: &str = include_str!("builtins.y");

#[macro_export]
macro_rules! plugin_fn {
    ($internal_name:ident, $external_name:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $external_name(args: *mut Vec<Value>) -> *mut SwResult<Value> {
            let args_ref: &mut Vec<Value> = args
                .as_mut()
                .expect("args given to schwift extern fn should never be null");

            let result: SwResult<Value> = $internal_name(args_ref);

            Box::into_raw(Box::new(result))
        }
    };
}

#[no_mangle]
pub static LIBSCHWIFT_ABI_COMPAT: u32 = 1;

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
    Modulus,
}

fn get_line<'a>(file: &'a str, err: &grammar::ParseError) -> &'a str {
    let mut count = 0;
    let mut last_newline = 0usize;

    for i in 0..file.len() {
        if file.is_char_boundary(i) {
            let symbol = &file[i..=i];

            if symbol == "\n" {
                count += 1;

                if count == err.location.line {
                    return &file[last_newline..i];
                }

                last_newline = i + 1;
            }
        }
    }

    panic!(
        "Got grammar error with invalid line number {}",
        err.location.line
    );
}

fn place_carat(err: &grammar::ParseError) -> String {
    let mut s = String::new();

    for _ in 0..err.location.column - 1 {
        s.push(' ');
    }

    s.push('^');

    s
}

pub fn compile(filename: &str) -> Vec<Statement> {
    let mut f = match File::open(filename) {
        Result::Ok(i) => i,
        Result::Err(_) => panic!("Failed to open file {}", filename),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Result::Ok(_) => {}
        Result::Err(_) => panic!("Failed to read file {}", filename),
    };

    parse_str(&s, filename)
}

fn parse_str(source: &str, filename: &str) -> Vec<Statement> {
    match grammar::file(source) {
        Ok(statements) => statements,
        Err(ref e) => {
            println!(
                "SYNTAX ERROR: {}:{}\n{}\n{}",
                filename,
                e.location.line,
                get_line(source, e),
                place_carat(e)
            );
            std::process::exit(1);
        }
    }
}

pub fn run_program(filename: &str, args: &[&str]) {
    let mut s = State::new();

    s.parse_args(args);

    let tokens = parse_str(BUILTINS, BUILTINS_FILE);

    match s.run(&tokens) {
        Ok(()) => {}
        Err(e) => e.panic(BUILTINS_FILE),
    }

    let tokens = compile(filename);

    match s.run(&tokens) {
        Ok(()) => {}
        Err(e) => e.panic(filename),
    }

    std::mem::forget(s);
}
