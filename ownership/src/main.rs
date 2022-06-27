fn owner_move_fixed_size_var() {
    // x is copied to x w/o clone() method.
    // because copying fixed size value is very fast.
    // and we DONT need to make x disabled.
    let x = 5;
    let y = x;
    println!("x is {}, y is {}", x, y);
}

fn owner_move_dynamic_size_var_string() {
    let s1 = String::from("helo");
    let s2 = s1;
    // compile error: s1 is moved to s2.
    //println!("{}, world", s1);
    println!("{}, world", s2);
}

fn owner_explicit_copy_by_clone_method() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(s: String) {
    println!("parameter is {}", s);
}

fn move_owner_by_calling_function() {
    let s = String::from("hello");
    takes_ownership(s);
    // cant use s from here because s is moved to function parameter
    //println!("after calling function {}", s);
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    return s;
}

fn take_and_gives_back(s: String) -> String {
    s
}

fn move_owner_by_return_value() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = take_and_gives_back(s2);
    println!("s1 is {}, s3 is {}", s1, s3);
}

fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);
    owner_move_fixed_size_var();
    owner_move_dynamic_size_var_string();
    owner_explicit_copy_by_clone_method();
    move_owner_by_calling_function();
    move_owner_by_return_value();
}
