fn main() {
    another_function(5);
    let y = {
        let x = 3;
        x + 1
    };
    let z = function_with_return_value();
    println!("The value of y is {}", y);
    println!("return value is {}", z);
}

fn another_function(x: i32) {
    println!("The value of x is {}", x);
}

fn function_with_return_value() -> i32 {
    5
}
