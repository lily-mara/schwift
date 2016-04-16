enum Value {
	Str(String),
	Int(i32),
	Float(f32),
	List(Vec<Value>),
}

struct Variable {
    value: Value,
    constant: bool,
}
