use std::env::*;

fn main() {
    println!("Hello, world!");
    let args = args();
    for arg in args {
        print!("{} ", arg);
    }
    println!();
}
