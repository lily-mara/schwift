use schwift::{value::Value error::{SwResult, ErrorKind}};

#[no_mangle]
pub fn matrix(args: &[Value]) -> SwResult<Value> {
    if let Value::Int(x) = args[0] {
        if let Value::Int(y) = args[1] {
            let mut mat = Vec::with_capacity(x as usize);
            for _ in 0..x {
                let mut row = Vec::with_capacity(y as usize);
                for _ in 0..y {
                    row.push(Value::new(0));
                }
                mat.push(row);
            }

            return Ok(Value::new(mat));
        }
    }

    Err(ErrorKind::UnexpectedType("Int, Int".into(), args[0].clone()))
}
