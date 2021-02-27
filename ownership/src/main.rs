fn main() {
    // ex 1
    let x = 5i32;
    make_copy(x);
    println!("{}", x);

    // ex 2
    let s = String::from("Hello");
    dont_take_ownership(&s);
    take_ownership(s);
    // println!("{}", s);

    // ex 3
    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);
    // println!("{}", s2)

    // ex 4
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn dont_take_ownership(some_string: &String) {
    println!("{}", some_string);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_number: i32) {
    println!("{}", some_number);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string 
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string 
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
