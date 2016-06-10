use super::Operator;
use std::cmp::Ordering;
use super::error::{ErrorKind, SwResult};
use super::statement::Statement;
use super::utils::perf;
use std;

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    List(Vec<Value>),
    Function(Vec<String>, Vec<Statement>),
}

impl From<i32> for Value {
    fn from(from: i32) -> Value {
        let _perf = perf("Value::From i32");
        Value::Int(from)
    }
}

impl From<f32> for Value {
    fn from(from: f32) -> Value {
        let _perf = perf("Value::From f32");
        Value::Float(from)
    }
}

impl From<bool> for Value {
    fn from(from: bool) -> Value {
        let _perf = perf("Value::From bool");
        Value::Bool(from)
    }
}

impl From<String> for Value {
    fn from(from: String) -> Value {
        let _perf = perf("Value::From String");
        Value::Str(from)
    }
}

impl From<&'static str> for Value {
    fn from(from: &'static str) -> Value {
        let _perf = perf("Value::From &'static str");
        Value::Str(from.into())
    }
}

impl<T> From<Vec<T>> for Value
    where T: Into<Value>
{
    fn from(from: Vec<T>) -> Value {
        let _perf = perf("Value::From T: Into<Value>");
        let mut buf = vec![];
        for i in from {
            buf.push(i.into());
        }

        Value::List(buf)
    }
}

impl Value {
    pub fn new<T>(val: T) -> Self
        where T: Into<Value>
    {
        let _perf = perf("Value::new");
        val.into()
    }

    pub fn print(&self) {
        let _perf = perf("Value::print");
        match *self {
            Value::Int(i) => print!("{}", i),
            Value::Float(i) => print!("{}", i),
            Value::Bool(i) => print!("{}", i),
            Value::Str(ref i) => print!("{}", i),
            Value::List(ref i) => print!("{:?}", i),
            Value::Function(_, ref body) => print!("{:?}", body),
        }
    }

    pub fn println(&self) {
        let _perf = perf("Value::println");
        self.print();
        println!("");
    }

    fn assert_f32(&self) -> SwResult<f32> {
        let _perf = perf("Value::assert_f32");
        match *self {
            Value::Float(f) => Ok(f),
            Value::Int(i) => Ok(i as f32),
            _ => Err(ErrorKind::UnexpectedType("float".to_string(), self.clone())),
        }
    }

    fn assert_bool(&self) -> SwResult<bool> {
        let _perf = perf("Value::assert_bool");
        match *self {
            Value::Bool(b) => Ok(b),
            _ => Err(ErrorKind::UnexpectedType("bool".to_string(), self.clone())),
        }
    }

    pub fn not(&mut self) -> SwResult<Value> {
        let _perf = perf("Value::not");
        match *self {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            _ => Err(ErrorKind::UnexpectedType("bool".to_string(), self.clone())),
        }
    }

    pub fn less_than(&self, other: &Value) -> SwResult<Value> {
        let _perf = perf("Value::less_than");
        Ok(Value::Bool(try!(self.assert_f32()) < try!(other.assert_f32())))
    }

    pub fn greater_than(&self, other: &Value) -> SwResult<Value> {
        let _perf = perf("Value::greater_than");
        Ok(Value::Bool(try!(self.assert_f32()) > try!(other.assert_f32())))
    }

    pub fn greater_than_equal(&self, other: &Value) -> SwResult<Value> {
        let _perf = perf("Value::greater_than_equal");
        Ok(Value::Bool(try!(self.assert_f32()) >= try!(other.assert_f32())))
    }

    pub fn less_than_equal(&self, other: &Value) -> SwResult<Value> {
        let _perf = perf("Value::less_than_equal");
        Ok(Value::Bool(try!(self.assert_f32()) <= try!(other.assert_f32())))
    }

    pub fn and(&self, other: &Value) -> SwResult<Value> {
        let _perf = perf("Value::and");
        Ok(Value::Bool(try!(self.assert_bool()) && try!(other.assert_bool())))
    }

    pub fn or(&self, other: &Value) -> SwResult<Value> {
        let _perf = perf("Value::or");
        Ok(Value::Bool(try!(self.assert_bool()) || try!(other.assert_bool())))
    }

    pub fn equals(&self, other: &Value) -> Value {
        let _perf = perf("Value::equals");
        Value::Bool(self.eq(other))
    }

    pub fn add(&self, other: &Value) -> SwResult<Value> {
        let _perf = perf("Value::add");
        match (self, other) {
            (&Value::Float(f1), &Value::Float(f2)) => Ok(Value::Float(f1 + f2)),
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 + i2)),
            (&Value::Float(f), &Value::Int(i)) |
            (&Value::Int(i), &Value::Float(f)) => Ok(Value::Float(i as f32 + f)),
            (&Value::Str(ref s1), &Value::Str(ref s2)) => {
                let mut new_buf = s1.clone();
                new_buf.push_str(s2);
                Ok(Value::Str(new_buf))
            }
            _ => {
                Err(ErrorKind::InvalidBinaryExpression(self.clone(), other.clone(), Operator::Add))
            }
        }
    }

    pub fn subtract(&self, other: &Value) -> SwResult<Value> {
        let _perf = perf("Value::subtract");
        match (self, other) {
            (&Value::Float(f1), &Value::Float(f2)) => Ok(Value::Float(f1 - f2)),
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 - i2)),
            (&Value::Float(f), &Value::Int(i)) => Ok(Value::Float(f - i as f32)),
            (&Value::Int(i), &Value::Float(f)) => Ok(Value::Float(i as f32 - f)),
            _ => {
                Err(ErrorKind::InvalidBinaryExpression(self.clone(),
                                                       other.clone(),
                                                       Operator::Subtract))
            }
        }
    }

    pub fn multiply(&self, other: &Value) -> SwResult<Value> {
        let _perf = perf("Value::multiply");
        match (self, other) {
            (&Value::Float(f1), &Value::Float(f2)) => Ok(Value::Float(f1 * f2)),
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 * i2)),
            (&Value::Float(f), &Value::Int(i)) |
            (&Value::Int(i), &Value::Float(f)) => Ok(Value::Float(i as f32 * f)),
            (&Value::Str(ref s), &Value::Int(i)) => {
                let mut new_buf = s.clone();
                for _ in 0..(i - 1) {
                    new_buf.push_str(s);
                }
                Ok(Value::Str(new_buf))
            }
            _ => {
                Err(ErrorKind::InvalidBinaryExpression(self.clone(),
                                                       other.clone(),
                                                       Operator::Multiply))
            }
        }
    }

    pub fn divide(&self, other: &Value) -> SwResult<Value> {
        let _perf = perf("Value::divide");
        match (self, other) {
            (&Value::Float(f1), &Value::Float(f2)) => Ok(Value::Float(f1 / f2)),
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 / i2)),
            (&Value::Float(f), &Value::Int(i)) => Ok(Value::Float(f / i as f32)),
            (&Value::Int(i), &Value::Float(f)) => Ok(Value::Float(i as f32 / f)),
            _ => {
                Err(ErrorKind::InvalidBinaryExpression(self.clone(),
                                                       other.clone(),
                                                       Operator::Divide))
            }
        }
    }

    pub fn shift_left(&self, other: &Value) -> SwResult<Value> {
        let _perf = perf("Value::shift_left");
        match (self, other) {
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 << i2)),
            _ => {
                Err(ErrorKind::InvalidBinaryExpression(self.clone(),
                                                       other.clone(),
                                                       Operator::ShiftLeft))
            }
        }
    }

    pub fn shift_right(&self, other: &Value) -> SwResult<Value> {
        let _perf = perf("Value::shift_right");
        match (self, other) {
            (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(i1 >> i2)),
            _ => {
                Err(ErrorKind::InvalidBinaryExpression(self.clone(),
                                                       other.clone(),
                                                       Operator::ShiftRight))
            }
        }
    }

    pub fn type_str<'a>(&self) -> &'a str {
        let _perf = perf("Value::type_str");
        match *self {
            Value::Str(_) => "string",
            Value::Int(_) => "int",
            Value::Bool(_) => "bool",
            Value::List(_) => "list",
            Value::Float(_) => "float",
            Value::Function(_, _) => "function",
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        let _perf = perf("Value::partial_cmp");
        if self == other {
            Option::Some(Ordering::Equal)
        } else {
            let (s, o) = match (self, other) {
                (&Value::Int(i), &Value::Float(f)) => (i as f32, f),
                (&Value::Float(f), &Value::Int(i)) => (f, i as f32),
                _ => return Option::None,
            };

            s.partial_cmp(&o)
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        let _perf = perf("Value::eq");
        match (self, other) {
            (&Value::Bool(b1), &Value::Bool(b2)) => b1 == b2,
            (&Value::Str(ref s1), &Value::Str(ref s2)) => s1 == s2,
            (&Value::List(ref l1), &Value::List(ref l2)) => l1 == l2,
            (&Value::Int(i1), &Value::Int(i2)) => i1 == i2,
            (&Value::Int(i), &Value::Float(f)) |
            (&Value::Float(f), &Value::Int(i)) => (i as f32 - f).abs() < std::f32::EPSILON,
            (&Value::Float(f1), &Value::Float(f2)) => (f1 - f2).abs() < std::f32::EPSILON,
            _ => false,
        }
    }
}
