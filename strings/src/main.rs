fn main() {
    let data = "initial data";

    let mut s = data.to_string();

    s.push_str(" test");

    let something = s.as_str();

    println!("{}", something);

    for c in something.chars() {
        println!("{}", c);
    }
}
