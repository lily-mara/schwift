extern crate schwift;

use schwift::{ Variable, Value, QUOTES };

fn main() {
    let x = Value::Str("Hello, Evans my favorite".to_string());
    let y = Variable::new_variable(x);

    let z = Value::Int(10);

    let b = y.multiply(z);
    //println!("Hello, {:?}!", x);
    println!("{:?}", b);
    println!("{}",QUOTES[2]);
}
