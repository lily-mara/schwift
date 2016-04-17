extern crate schwift;

use schwift::{ Variable, parse_file };
use schwift::schwift_grammar::{ file };

fn main() {
    //println!("{:?}", statement("x squanch 10"));
    //println!("{:?}", statement("squanch x"));
    println!("{:?}", parse_file("hello.y"));
    //println!("{:?}", assignment("x squanch 10"));
    //println!("{:?}", deletion("squanch x"));

}
