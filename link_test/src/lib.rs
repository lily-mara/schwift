use schwift::{
    error::{ErrorKind, SwResult},
    plugin_fn,
    value::{Type, Value},
};

plugin_fn!(matrix_internal, matrix);

fn matrix_internal(args: &mut Vec<Value>) -> SwResult<Value> {
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

    Err(ErrorKind::UnexpectedType {
        expected: Type::List,
        actual: args[0].get_type(),
    })
}
