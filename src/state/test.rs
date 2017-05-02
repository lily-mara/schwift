use ::state::State;
use ::statement::StatementKind as Kind;
use ::statement::Statement;
use ::value::Value;
use ::expression::Expression as Exp;
use ::error::ErrorKind as EKind;
use ::grammar;

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

#[test]
fn test_is_prime() {
    let mut state = State::new();

    let code = grammar::file(r#"
    isPrime(x) :<
        i squanch 2

        while (i less x) :<
            if ((x % i) == 0) :<
                return morty
            >:
            i squanch (i + 1)
        >:

        return rick
    >:

    ten squanch isPrime(10)
    two squanch isPrime(2)
    eighteen squanch isPrime(18)
    fourHundredFourty squanch isPrime(440)
    big squanch isPrime(524287)
    "#)
        .unwrap();

    state.run(&code).unwrap();
    assert_eq!(*state.get("ten").unwrap(), Value::new(false));
    assert_eq!(*state.get("two").unwrap(), Value::new(true));
    assert_eq!(*state.get("eighteen").unwrap(), Value::new(false));
    assert_eq!(*state.get("fourHundredFourty").unwrap(), Value::new(false));
    assert_eq!(*state.get("big").unwrap(), Value::new(true));
}

#[test]
fn test_modulus() {
    let mut state = State::new();

    let code = grammar::file(r#"
    x squanch (50 % 4)
    "#)
        .unwrap();

    state.run(&code).unwrap();
    assert_eq!(*state.get("x").unwrap(), Value::new(2));
}

#[test]
fn test_early_return_if() {
    let mut state = State::new();

    let code = grammar::file(r#"
    small(x) :<
        if (x less 100) :<
            return rick
        >:
        return morty
    >:

    y squanch small(10)
    z squanch small(10000)
    a squanch small(140)
    b squanch small(0)
    "#)
        .unwrap();

    state.run(&code).unwrap();
    assert_eq!(*state.get("y").unwrap(), Value::new(true));
    assert_eq!(*state.get("z").unwrap(), Value::new(false));
    assert_eq!(*state.get("a").unwrap(), Value::new(false));
    assert_eq!(*state.get("b").unwrap(), Value::new(true));
}

#[test]
fn test_early_return_while() {
    let mut state = State::new();

    let code = grammar::file(r#"
    small(x) :<
        i squanch 0
        while (i less 100) :<
            if (x less 100) :<
                return rick
            >:
            i squanch (i + 1)
        >:
        return morty
    >:

    y squanch small(10)
    z squanch small(10000)
    a squanch small(140)
    b squanch small(0)
    "#)
        .unwrap();

    state.run(&code).unwrap();
    assert_eq!(*state.get("y").unwrap(), Value::new(true));
    assert_eq!(*state.get("z").unwrap(), Value::new(false));
    assert_eq!(*state.get("a").unwrap(), Value::new(false));
    assert_eq!(*state.get("b").unwrap(), Value::new(true));
}
