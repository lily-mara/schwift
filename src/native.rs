use super::value::Value;
use super::error::{SwResult, ErrorKind};

pub fn multiply(args: &[Value]) -> SwResult<Value> {
    if let Value::Str(ref s) = args[0] {
        if let Value::Int(i) = args[1] {
            let mut new_str = String::with_capacity(i as usize);

            for _ in 0..i {
                new_str.push_str(s);
            }

            return Ok(new_str.into());
        }
    }

    Err(ErrorKind::UnexpectedType("string".into(), args[0].clone()))
}
