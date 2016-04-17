use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub mod schwift_grammar;

#[derive(Debug,Clone)]
pub enum Value {
	Str(String),
	Int(i32),
	Float(f32),
    Bool(bool),
	List(Vec<Value>),
}

#[derive(Clone)]
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

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equality,
}

#[derive(Debug)]
pub enum Expression {
    Variable(String),
    OperatorExpression(Box<Expression>, Operator, Box<Expression>),
    Value(Value),
    ListIndex(String, i32),
}

#[derive(Debug)]
pub enum Statement {
    Assignment(String, Expression),
    Delete(String),
    Print(Expression),
    ListNew(String),
    ListAppend(String, Expression),
    ListAssign(String, i32, Value),
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

impl <T>Op<T> {
    fn unwrap(self) -> T {
        match self {
            Op::Ok(x) => x,
            Op::TypeError(x, y) => panic!("Cannot combine {:?} and {:?}", x, y)
        }
    }
}

impl State {
    fn expression_to_variable(&self, exp: Expression) -> Variable {
        match exp {
            Expression::Variable(ref s) => {
                if self.symbols.contains_key(s) {
                    let y = &(self.symbols);
                    let x = y.get(s).unwrap();
                    x.clone()
                } else {
                    panic!("Tried to use variable {} before assignment", s)
                }
            }
            Expression::OperatorExpression(a, o, b) => {
                let x = self.expression_to_variable(*a);
                let y = self.expression_to_variable(*b);
                Variable::new_variable(match o {
                    Operator::Add => x.add(y.value).unwrap(),
                    Operator::Subtract => x.subtract(y.value).unwrap(),
                    Operator::Multiply => x.multiply(y.value).unwrap(),
                    Operator::Divide => x.divide(y.value).unwrap(),
                    _=> panic!("Unimplemented"),
                })
            }
            Expression::Value(v) => Variable::new_variable(v),
        }
    }

    fn assign(&mut self, str: String, exp: Expression) {
        let v = self.expression_to_variable(exp);
        self.symbols.insert(str, v);
    }

    fn delete(&mut self, str: String) {
        self.symbols.remove(&str);
    }

    fn print(&mut self, exp: Expression) {
        let x = self.expression_to_variable(exp);
        x.println();
    }

    pub fn run(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            match statement {
                Statement::Assignment(i, j) => self.assign(i, j),
                Statement::Delete(i) => self.delete(i),
                Statement::Print(i) => self.print(i),
            }
        }
    }

    pub fn new() -> State {
        State {
            symbols:HashMap::new()
        }
    }
}

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
                    for _ in 0..(j - 1) {
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

pub fn parse_file(filename: &str) ->  Result<Vec<Statement>, schwift_grammar::ParseError> {
    let mut f = match File::open(filename){
        Result::Ok(i) => i,
        Result::Err(_) => panic!("failed to open file"),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Result::Ok(_) => {},
        Result::Err(_) => panic!("failed to read file"),
    };
    schwift_grammar::file(&s)
}

pub fn run_program(filename: &str) {
    let mut s = State::new();
    s.run(parse_file(filename).unwrap());
}
