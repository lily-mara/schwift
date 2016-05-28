extern crate schwift;
extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("The Schwift interpreter")
        .version("0.1")
        .author("Nate Mara <natemara@gmail.com>")
        .about("The canonical interpreter for the schwift programming language. Use at your own \
                risk.")
        .arg(Arg::with_name("SOURCE")
            .value_name("FILE")
            .help("The schwift source file that you want to interpret")
            .takes_value(true)
            .required(true))
        .get_matches();


    schwift::run_program(matches.value_of("SOURCE").unwrap());
}
