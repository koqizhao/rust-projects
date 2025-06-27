macro_rules! show {
    ($name: tt) => {
        println!("show: {}", $name);
    };
}

fn main() {
    println!("Hello, world!");
    show!("Ok");
    let x = 99;
    show!(x);
}
