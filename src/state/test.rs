pub use super::State;
pub use super::super::statement::StatementKind as Kind;
pub use super::super::statement::Statement;
pub use super::super::value::Value;
pub use super::super::expression::Expression as Exp;
pub use super::super::error::ErrorKind as EKind;

#[test]
fn test_assignment_adds_to_symbol_table() {
    let mut state = State::new();

    let statement = Statement::tnew(Kind::assignment("x", 10));
    state.execute(&statement).unwrap();
    assert_eq!(state.symbols.get("x"), Some(&(Value::new(10))));
}

#[test]
fn test_remove_from_symbol_table() {
    let mut state = State::new();

    let statement = Statement::tnew(Kind::assignment("x", 10));
    state.execute(&statement).unwrap();
    assert_eq!(state.symbols.get("x"), Some(&(Value::new(10))));

    let delete = Statement::tnew(Kind::delete("x"));
    state.execute(&delete).unwrap();
    assert_eq!(state.symbols.get("x"), None);
}

#[test]
fn test_get_returns_value_if_present() {
    let mut state = State::new();

    state.assign("x".to_string(), &Exp::new(10)).unwrap();
    assert_eq!(*state.get("x").unwrap(), Value::new(10));
}

#[test]
fn test_get_returns_error_if_not_present() {
    let state = State::new();

    assert_eq!(state.get("x"), Err(EKind::UnknownVariable("x".to_string())));
}
