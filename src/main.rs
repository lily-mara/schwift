extern crate schwift;

use schwift::{ Variable, Value };
use schwift::schwift_grammar::{ value, statement, identifier, assignment, deletion };

fn main() {
    println!("{:?}", value("\"hello!\""));
    println!("{:?}", value("10"));
    println!("{:?}", value("10.5"));
    println!("{:?}", statement("x squanch 10"));
    println!("{:?}", statement("squanch x"));
    println!("{:?}", assignment("x squanch 10"));
    println!("{:?}", deletion("squanch x"));
}
