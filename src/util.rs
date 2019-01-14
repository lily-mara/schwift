use crate::value::Value;
use std::fmt;

pub fn slice_format<T>(x: &[T]) -> String
where
    T: fmt::Display,
{
    let mut s: String = "[".into();

    for (idx, val) in x.iter().enumerate() {
        s.push_str(&format!("{}", val));
        if idx != x.len() - 1 {
            s.push_str(", ");
        }
    }
    s.push(']');

    s
}

pub fn slice_value_format(x: &[Value]) -> String {
    let mut s: String = "[".into();

    for (idx, val) in x.iter().enumerate() {
        if let Value::Str(ref str) = *val {
            s.push('"');
            s.push_str(&str);
            s.push('"');
        } else {
            s.push_str(&format!("{}", val));
        }
        if idx != x.len() - 1 {
            s.push_str(", ");
        }
    }
    s.push(']');

    s
}
