#[derive(Debug)]
pub enum Value {
	Str(String),
	Int(i32),
	Float(f32),
    Bool(bool),
	List(Vec<Value>),
}

pub struct Variable {
    value: Value,
    constant: bool,
}

impl Variable {
    pub fn new_variable(value: Value) -> Variable {
        Variable{ value: value, constant: false }
    }

    pub fn new_constant(value: Value) -> Variable {
        Variable{ value: value, constant: true }
    }

    pub fn print(&self) {
        match &self.value {
            &Value::Int(i) => print!("{}", i),
            &Value::Float(i) => print!("{}", i),
            &Value::Bool(i) => print!("{}", i),
            &Value::Str(ref i) => print!("{}", i),
            &Value::List(ref i) => print!("{:?}", i),
        }
    }

    pub fn println(&self) {
        self.print();
        println!("");
    }

    pub fn assign(&mut self, value: Value) {

    }

    pub fn add(&self, value: Value) -> Value {

    }

    pub fn subtract(&self, value: Value) -> Value {

    }

    pub fn multipy(&self, value: Value) -> Value {

    }

    pub fn divide(&self, value: Value) -> Value {

    }
}
