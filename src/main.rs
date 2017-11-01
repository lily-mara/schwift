extern crate schwift;
extern crate clap;

use clap::{Arg, App, AppSettings};

fn main() {
    let matches = App::new("The Schwift interpreter")
        .version("0.1")
        .setting(AppSettings::TrailingVarArg)
        .author("Nate Mara <natemara@gmail.com>")
        .about(
            "The canonical interpreter for the schwift programming language. Use at your own \
                risk.",
        )
        .arg(
            Arg::with_name("SOURCE")
                .value_name("FILE")
                .help("The schwift source file that you want to interpret")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("args")
                .help("Args to pass to the program")
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();

    let args = match matches.values_of("args") {
        Some(x) => x.collect(),
        None => Vec::new(),
    };


    schwift::run_program(matches.value_of("SOURCE").unwrap(), &args);
}
