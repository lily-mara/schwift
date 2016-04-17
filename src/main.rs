extern crate schwift;

use schwift::{ Variable, Value };
use schwift::schwift_grammar::{ line };

fn main() {
    //println!("{:?}", statement("x squanch 10"));
    //println!("{:?}", statement("squanch x"));
    println!("{:?}", line("show me what you got x\n"));
    //println!("{:?}", assignment("x squanch 10"));
    //println!("{:?}", deletion("squanch x"));
}
