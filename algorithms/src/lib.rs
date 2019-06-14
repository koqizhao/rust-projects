pub mod sort;

#[cfg(test)]
mod tests {

    use super::sort::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let mut l = Vec::new();
        l.push(1);
        l[0] = 2;
        l.push(3);
        l.push(1);
        print_vec(l.as_ref());
        quick_sort(&mut l);
        print_vec(l.as_ref());
    }

    #[test]
    fn it_works_2() {
        assert_eq!(2 + 2, 4);
        let mut l = Vec::new();
        l.push(1);
        l[0] = 2;
        l.push(3);
        l.push(1);
        print_vec(l.as_ref());
        bubble_sort(&mut l);
        print_vec(l.as_ref());
    }

    #[test]
    fn it_works_3() {
        assert_eq!(2 + 2, 4);
        let mut l = Vec::new();
        l.push(1);
        l[0] = 2;
        l.push(3);
        l.push(1);
        print_vec(l.as_ref());
        selection_sort(&mut l);
        print_vec(l.as_ref());
    }

    fn print_vec(l: &Vec<i32>) {
        for e in l {
            println!("{} ", e);
        }
        println!();
    }
}
