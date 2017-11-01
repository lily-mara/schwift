extern crate assert_cli;

use std::io::Read;
use std::fs::File;

#[test]
fn test_hello_world() {
    assert_cli::Assert::main_binary()
        .with_args(&["examples/hello.y"])
        .stdin("Nate")
        .stdout()
        .contains("Enter your name: ")
        .and()
        .stdout()
        .contains("Hello, Nate")
        .unwrap();
}

#[test]
fn test_brainfuck() {
    let mut f = File::open("examples/hello.brainfuck").unwrap();
    let mut s = String::new();

    f.read_to_string(&mut s).unwrap();

    assert_cli::Assert::main_binary()
        .with_args(&["examples/brainfuck.y"])
        .stdin(&s)
        .stdout()
        .contains("Hello World!")
        .unwrap();
}
