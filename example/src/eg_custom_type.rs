#![allow(dead_code)]

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_enum() {
        let e = TestEnum::XX;
        match e {
            TestEnum::XX => {
                println!("XX");
            }
            _ => {
                println!("else");
            }
        }
    }
}

enum TestEnum {
    XX,
    YY,
}
