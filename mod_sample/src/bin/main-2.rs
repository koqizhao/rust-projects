
use mod_sample::*;

fn main() {
    println!("Hello, world!");
    mod1::mod11::say_hello();
    mod1::mod12::say_hello_2();
    mod2::say_ok();
    mod3::mod32::say_mod32();
}
