fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("mutable");
    change_any_through_reference(&mut s2);
    println!("after change to '{}'", s2);

    make_variable_refer_to_same_variable_after_dropped();

    make_reference_mixed();

    println!("no dangle {}", no_dangle());
}

// refer string w/o moving ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}

// will compile error
//fn change_any_through_reference(some_string: &String) {
//    some_string.push_str("abc");
//}

// OK
fn change_any_through_reference(some_string: &mut String) {
    some_string.push_str("abc");
}

// will compile error
//fn make_variable_refer_to_same_variable_more_than_once() {
//    let mut s = String::from("hello");
//    let r1 = &mut s;
//    let r2 = &mut s;  // error by here
//    println!("{}, {}", r1, r2);
//}

fn make_variable_refer_to_same_variable_after_dropped() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(" by r1");
        println!("{}", r1);
    } // "r1" is dropped here.

    let r2 = &mut s; // its ok because preceding r1 is already dropped
    r2.push_str(" by r2");
    println!("{}", r2);
}

fn make_reference_mixed() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // cant create reference by mutable and immutable
    //let r3 = &mut s;
    println!("r1 is {},", r1);
    println!("r2 is {},", r2);
}

// compile error
//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
