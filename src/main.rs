extern crate schwift;

use std::env;

fn main() {
    schwift::run_program(&env::args().nth(1).unwrap());
}
