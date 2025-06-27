#[cfg(test)]
mod tests {

    #[derive(Debug, PartialEq, Clone)]
    struct CustomType {
        value: i32,
    }

    #[test]
    fn test_eq() {
        let a = CustomType { value: 5 };
        let b = CustomType { value: 5 };
        assert_eq!(a, b, "Values should be equal");

        let ra = &a;
        let rb = &b;
        assert_eq!(ra, rb, "References should be equal");

        println!("ref: {}, value: {}, *: {}", ra == rb, a == b, *ra == *rb);
        println!("ra: {:p}, rb: {:p}", ra, rb);
    }

    #[test]
    fn test_eq2() {
        let a = [1, 2, 3];
        let b = [1, 2, 3];
        assert_eq!(a, b, "Values should be equal");
        let c = a;
        let d = a;
        assert_eq!(c, d, "Arrays should be equal");
    }

    #[test]
    fn test_eq3() {
        let a = [CustomType { value: 1 }, CustomType { value: 2 }];
        let b = a.clone();
        assert_eq!(a, b, "Arrays of CustomType should be equal");
    }
}
