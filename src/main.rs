extern crate schwift;

use schwift::{ Variable, Value };

fn main() {
    let x = Value::Int(5);
    let y = Variable::new_variable(x);
    //println!("Hello, {:?}!", x);
    y.println();
}
