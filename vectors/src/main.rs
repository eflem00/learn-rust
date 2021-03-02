fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(15);

    let mut vec2 = vec!["some val"];
    match vec2.pop() {
        Some(val) => println!("{}", val),
        None => println!("no val")
    }

    let mut v = vec![1, 2, 3, 4, 5];
    {
        let third: i32 = v[2];
        println!("The third element is {}", third);
    }
    v.push(6);
    println!("The third element is {}", v[2]);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in row {
        match cell {
            SpreadsheetCell::Int(val) => println!("{}", val),
            _ => ()
        }
    }
}
