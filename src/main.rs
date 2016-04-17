extern crate schwift;

use schwift::run_program;
use std::env;

fn main() {
    run_program(&env::args().nth(1).unwrap());
}
