fn array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array  a is {:?}", a);
}

fn types() {
    let t = true;
    println!("The value of t is: {}", t);
}

fn touple() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    let x1 = tup.0;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of x1 is: {}", x1);
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const MAX_PONITS: u32 = 100_000;
    println!("The value of MAX_PONITS is: {}", MAX_PONITS);

    shadowing();
    types();
    touple();
    array();
}
