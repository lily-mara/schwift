extern crate schwift;

use schwift::{ Variable, Value };
use schwift::schwift_grammar::{ file };

fn main() {
    //println!("{:?}", statement("x squanch 10"));
    //println!("{:?}", statement("squanch x"));
    println!("{:?}", file("show me what you got x\n"));
    //println!("{:?}", assignment("x squanch 10"));
    //println!("{:?}", deletion("squanch x"));
}
