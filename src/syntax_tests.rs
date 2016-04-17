use super::schwift_grammar;
use super::{ Value, Statement, Expression };

#[test]
fn test_raw_int() {
    let l = schwift_grammar::int("3").unwrap();
    assert_eq!(l, 3)
}

#[test]
fn test_raw_string() {
    let l = schwift_grammar::string("\"hello!\"").unwrap();
    assert_eq!(l, Value::Str("hello!".to_string()))
}

#[test]
fn test_expression_string() {
    let l = schwift_grammar::expression("\"hello!\"").unwrap();
    assert_eq!(l, Expression::Value(Value::Str("hello!".to_string())));
}
