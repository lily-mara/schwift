extern crate schwift;

use schwift::{ Variable, Value };

fn main() {
    let x = Value::Int(5);
    let y = Variable::new_variable(x);

    let z = Value::Float(5.0);

    let b = y.add(z);
    //println!("Hello, {:?}!", x);
    println!("{:?}", b);
}
