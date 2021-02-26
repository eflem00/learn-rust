use std::io;

fn main() {
    println!("Please provide an n value");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("input not provided");

        let n: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        if n == 0 || n == 1 {
            println!("fib: {}", n);
            continue;
        }

        let mut head = 0;
        let mut tail = 1;

        for _ in 0..n {
            let temp = head;
            head = head + tail;
            tail = temp;
        }

        println!("fib: {}", head);
    }
}
