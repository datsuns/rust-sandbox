fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_by_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn string_slice() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} is splited to {},{}", s, hello, world);
}

//fn cause_compile_error_by_using_slice_of_string() {
//  // 可変借用
//    let mut s = String::from("hello world");
//  // 不変借用
//    let word = first_word_by_slice(&s);
//  // 不変借用が存在しておりさらなる可変借用はできない
//    s.clear();
//    println!("the first word is:{}", word);
//}

fn main() {
    let s = String::from("Hello, world!");
    first_word(&s);
    string_slice();
    let f = first_word_by_slice(&s);
    println!("{} of first word is {}", s, f);
}
