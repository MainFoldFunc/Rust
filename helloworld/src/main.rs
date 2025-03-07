fn main() {
    /*
            println!("Hello, world!");
            let mut x = 32;
            let mut y = 48;
            println!("x + y = {}", x + y);
            println!("x - y = {}", x - y);
            println!("x * y = {}", x * y);
            println!("x / y = {}", x / y);
            x = 50;
            y = 50;
            println!("Change x and y to 50");
            assert_eq!(x, y);
            if x == y {
                println!("x and y are the same");
            } else {
                println!("x and y are not the same");
            }
            for i in 0..5 {
                x += i;
                println!("Added {} to x, now x = {}", i, x);
            }
            println!("Adding {} to x", y);
            println!("Now x = {}", x);
            abs(x);
            println!("{}' fibonaci number is {}", 30, fibo(30));
            let mut x: i32 = 40;
            borr(&mut x);

            let pi = std::f64::consts::PI;

            println!("PI = {}", pi);
        let arr = [10, 20, 30];
        let firstarr = arr[0];
        println!("First element of arr is {}", firstarr);

        for i in arr.iter() {
            println!("{}", i);
        }
        let vec.c = vec![10, 20, 30];
        for (index, element) in vec.c.iter().enumerate() {
            println!("The {} index is {}", index, element);
        }

        let ints = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        println!("The debug to print a list {:?}", ints);

        let slice2 = &ints[0..2];
        println!("{:?}", slice2);
        let slice3 = &ints[3..];
        println!("{:?}", slice3);

        let first = slice3.get(0);
        println!("The last elememnt of the {:?} is {:?}", slice3, first);
        let lastno = slice3.get(100);
        println!("The none element of the {:?} is {:?}", slice3, lastno);
    */
    let mut vec = Vec::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);
    vec.push(40);

    let first = vec[0];
    let mfirst = vec.get(0);
    println!("{}", first);
    println!("{:?}", mfirst);
}
/*
fn abs(x: i32) -> i32 {
    if x > 0 {
        x
    }else {
        -x
    }
}
fn fibo (x: i64) -> i64{
    if x <= 2 {
        return x;
    }else {
        return fibo(x - 1) + fibo(x - 2)
    }
}

fn borr(x: &mut i32) {
    *x = *x * *x;
}
*/
