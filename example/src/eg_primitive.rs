#![allow(unused)]

#[cfg(test)]
mod tests {

    #[test]
    fn test_array() {
        let x = 10;
        //let arr = [3; x];
        //let arr: [i32; x] = [0];
        let b: Box<i32> = Box::new(10);

        let mut x = X;
        let y = Y { x, y: X };

        x = y.x;
        println!("{:?}", y.y);

        println!("\nnested array\n");
        let arr: [[i32; 1]; 1] = [[0]];
        println!("{:?}", arr);
    }

    #[derive(Debug)]
    struct X;

    struct Y {
        x: X,
        y: X,
    }
}
