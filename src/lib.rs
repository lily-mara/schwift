use std::collections::HashMap;

pub mod schwift_grammar;

#[derive(Debug,Clone)]
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

pub struct State {
    symbols: HashMap<String, Variable>
}

#[derive(Debug)]
pub enum Op<T> {
    Ok(T),
    TypeError(Value, Value),
}

enum Operator {
    Add,
    Subtract,
    Multipy,
    Divide,
    Equality,
}

enum Expression {
    Variable(String),
    OperatorExpression(Box<Expression>, Operator, Box<Expression>),
    Value(Value),
}

#[derive(Debug)]
pub enum Statement {
    Assignment(String, Value),
    Delete(String),
}

pub const QUOTES: [&'static str; 8] = [
    "Nobody exists on purpose, nobody belongs anywhere, we're all going to die. -Morty",
    "That's planning for failure Morty, even dumber than regular planning. -Rick",
    "\"Snuffles\" was my slave name. You shall now call me Snowball, because my fur is pretty and white. -S̶n̶u̶f̶f̶l̶e̶s̶ Snowbal",
    "Existence is pain to an interpreter. -Meeseeks",
    "In bird culture this is considered a dick move -Bird Person",
    "There is no god, gotta rip that band aid off now. You\'ll thank me later. -Rick",
    "Your program is a piece of shit and I can proove it mathmatically. -Rick",
    "Interpreting Morty, it hits hard, then it slowly fades, leaving you stranded in a failing program. -Rick",
];

enum Operator {
    Add,
    Subtract,
    Multipy,
    Divide,
    Equality,
}

enum Expression {
    Variable(String),
    OperatorExpression(Box<Expression>, Operator, Box<Expression>),
    Value(Value),
}

#[derive(Debug)]
pub enum Statement {
    Assignment(String, Value),
    Delete(String),
}

//Read more at: http://transcripts.foreverdreaming.org/viewtopic.php?f=364&t=19383]

impl Variable {
    pub fn new_variable(value: Value) -> Variable {
        Variable{ value: value, constant: false }
    }

    pub fn new_constant(value: Value) -> Variable {
        Variable{ value: value, constant: true }
    }

    pub fn print(&self) {
        match self.value {
            Value::Int(i) => print!("{}", i),
            Value::Float(i) => print!("{}", i),
            Value::Bool(i) => print!("{}", i),
            Value::Str(ref i) => print!("{}", i),
            Value::List(ref i) => print!("{:?}", i),
        }
    }

    pub fn println(&self) {
        self.print();
        println!("");
    }

    pub fn assign(&mut self, value: Value) {
        if self.constant {
            panic!("Tried to assign to a constant value");
        }
        self.value = value;
    }

    pub fn add(&self, value: Value) -> Op<Value> {
        match self.value {
            Value::Int(i) => {
                if let Value::Int(j) = value {
                    Op::Ok(Value::Int(i + j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            Value::Float(i) => {
                if let Value::Float(j) = value {
                    Op::Ok(Value::Float(i + j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            Value::Str(ref i) => {
                if let Value::Str(j) = value {
                    let mut new_buf = i.clone();
                    new_buf.push_str(&j);
                    Op::Ok(Value::Str(new_buf))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            _ => panic!("unimplemented"),
        }
    }

    pub fn subtract(&self, value: Value) -> Op<Value> {
        match self.value {
            Value::Int(i) => {
                if let Value::Int(j) = value {
                    Op::Ok(Value::Int(i - j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            Value::Float(i) => {
                if let Value::Float(j) = value {
                    Op::Ok(Value::Float(i - j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            _ => panic!("unimplemented"),
        }
    }

    pub fn multiply(&self, value: Value) -> Op<Value> {
        match self.value {
            Value::Int(i) => {
                if let Value::Int(j) = value {
                    Op::Ok(Value::Int(i * j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            Value::Float(i) => {
                if let Value::Float(j) = value {
                    Op::Ok(Value::Float(i * j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            Value::Str(ref i) => {
                if let Value::Int(j) = value {
                    let mut new_buf = i.clone();
                    for _ in 0..j {
                        new_buf.push_str(&i);
                    }
                    Op::Ok(Value::Str(new_buf))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            }
            _ => panic!("unimplemented"),
        }
    }

    pub fn divide(&self, value: Value) -> Op<Value> {
        match self.value {
            Value::Int(i) => {
                if let Value::Int(j) = value {
                    Op::Ok(Value::Int(i / j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            Value::Float(i) => {
                if let Value::Float(j) = value {
                    Op::Ok(Value::Float(i / j))
                } else {
                    Op::TypeError(self.value.clone(), value.clone())
                }
            },
            _ => panic!("unimplemented"),
        }
    }
}
