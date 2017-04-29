use super::{Operator, grammar};
use super::value::Value;
use super::error::{ErrorKind, SwResult};
use super::state::State;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Variable(String),
    OpExp(Box<Expression>, Operator, Box<Expression>),
    Value(Value),
    ListIndex(String, Box<Expression>),
    ListLength(String),
    Not(Box<Expression>),
    Eval(Box<Expression>),
    FunctionCall(String, Vec<Expression>),
}

impl<T> From<T> for Expression
    where T: Into<Value>
{
    fn from(fr: T) -> Expression {
        Expression::Value(fr.into())
    }
}

impl Expression {
    pub fn new<T>(from: T) -> Expression
        where T: Into<Expression>
    {
        from.into()
    }

    pub fn variable<S>(name: S) -> Expression
        where S: Into<String>
    {
        Expression::Variable(name.into())
    }

    pub fn list_length<S>(name: S) -> Expression
        where S: Into<String>
    {
        Expression::ListLength(name.into())
    }

    pub fn operator<L, R>(left: L, op: Operator, right: R) -> Expression
        where L: Into<Expression>,
              R: Into<Expression>
    {
        Expression::OpExp(Box::new(left.into()), op, Box::new(right.into()))
    }

    pub fn not<E>(expr: E) -> Expression
        where E: Into<Expression>
    {
        Expression::Not(Box::new(expr.into()))
    }

    pub fn eval<E>(expr: E) -> Expression
        where E: Into<Expression>
    {
        Expression::Eval(Box::new(expr.into()))
    }

    pub fn list_index<S, E>(name: S, index: E) -> Expression
        where S: Into<String>,
              E: Into<Expression>
    {
        Expression::ListIndex(name.into(), Box::new(index.into()))
    }

    pub fn value<V>(val: V) -> Expression
        where V: Into<Value>
    {
        Expression::Value(val.into())
    }

    pub fn evaluate(&self, state: &mut State) -> SwResult<Value> {
        match *self {
            Expression::Variable(ref name) => state.get(name).map(|x| x.clone()),
            Expression::OpExp(ref left_exp, ref operator, ref right_exp) => {
                let left = try!(left_exp.evaluate(state));
                let right = try!(right_exp.evaluate(state));
                match *operator {
                    Operator::Add => left.add(&right),
                    Operator::Subtract => left.subtract(&right),
                    Operator::Multiply => left.multiply(&right),
                    Operator::Divide => left.divide(&right),
                    Operator::Equality => Ok(left.equals(&right)),
                    Operator::LessThan => left.less_than(&right),
                    Operator::GreaterThan => left.greater_than(&right),
                    Operator::LessThanEqual => left.less_than_equal(&right),
                    Operator::GreaterThanEqual => left.greater_than_equal(&right),
                    Operator::ShiftLeft => left.shift_left(&right),
                    Operator::ShiftRight => left.shift_right(&right),
                    Operator::And => left.and(&right),
                    Operator::Or => left.or(&right),
                }
            }
            Expression::Value(ref v) => Ok(v.clone()),
            Expression::ListIndex(ref var_name, ref e) => state.list_index(var_name, e),
            Expression::Not(ref e) => try!(e.evaluate(state)).not(),
            Expression::ListLength(ref var_name) => {
                let value = try!(state.get(var_name));
                match *value {
                    Value::List(ref list) => Ok(Value::Int(list.len() as i32)),
                    Value::Str(ref s) => Ok(Value::Int(s.len() as i32)),
                    _ => Err(ErrorKind::IndexUnindexable(value.clone())),
                }
            }
            Expression::Eval(ref exp) => {
                let inner_val = try!(exp.evaluate(state));
                if let Value::Str(ref inner) = inner_val {
                    match grammar::expression(inner) {
                        Ok(inner_evaled) => inner_evaled.evaluate(state),
                        Err(s) => Err(ErrorKind::SyntaxError(s)),
                    }
                } else {
                    Err(ErrorKind::UnexpectedType("string".to_string(), inner_val))
                }
            }
            Expression::FunctionCall(ref name, ref args) => state.call_function(name, args),
        }
    }

    pub fn try_bool(&self, state: &mut State) -> SwResult<bool> {
        let value = try!(self.evaluate(state));
        if let Value::Bool(x) = value {
            Ok(x)
        } else {
            Err(ErrorKind::UnexpectedType("bool".to_string(), value))
        }
    }

    pub fn try_int(&self, state: &mut State) -> SwResult<i32> {
        let value = try!(self.evaluate(state));
        if let Value::Int(x) = value {
            Ok(x)
        } else {
            Err(ErrorKind::UnexpectedType("int".to_string(), value))
        }
    }
}
