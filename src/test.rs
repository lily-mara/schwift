pub use super::*;
pub use super::StatementKind as Kind;

describe! state {
    before_each {
        let mut state = State::new();
    }

    it "should add symbol to table when processing assignment statement" {
        let statement = Statement::new(Kind::assignment("x", 10));
        state.execute(&statement).unwrap();
        assert_eq!(state.symbols.get("x"), Some(&(10.into())));
    }

    it "should remove symbol from table when processing deletion statement" {
        let statement = Statement::new(Kind::assignment("x", 10));
        state.execute(&statement).unwrap();
        assert_eq!(state.symbols.get("x"), Some(&(10.into())));

        let delete = Statement::new(Kind::delete("x"));
        state.execute(&delete).unwrap();
        assert_eq!(state.symbols.get("x"), None);
    }
}
