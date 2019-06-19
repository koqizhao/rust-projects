use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_print() {
        println!("Hello, world!");

        let x = TestData {
            x: 1,
            y: String::from("Hello, world!")
        };

        eprintln!("{0}, {1}", x, "ok");
        println!("{}", x.to_string());

        let l = vec![1, 2, 3, 4, 5];
        println!("{}", to_string(&l));
    }
}

#[allow(dead_code)] 
struct TestData {
    x: i32,
    y: String
}

impl Display for TestData {

    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }

}

#[allow(dead_code)] 
fn to_string<T: ToString>(l: &Vec<T>) -> String {
    let mut s = String::from("[");
    for (i, e) in l.iter().enumerate() {
        s += " ";
        s += &e.to_string();
        if i != l.len() - 1 {
            s += ","
        }
    }
    s += " ]";
    return s;
}
