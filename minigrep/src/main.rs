use std::env::*;

mod lib;

use lib::*;

fn main() {
    let result = run(args()).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    println!("The grep result: \n");
    for part in result {
        println!("{}", part);
    }
}
