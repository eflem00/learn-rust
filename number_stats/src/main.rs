use std::collections::HashMap;
use std::io;

fn main() {
    println!("Enter integers delimited by whitespace");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut input_nums = input.split_whitespace();

    if input_nums.size_hint().0 == 0 {
        match input_nums.next() {
            Some(val) => match val.parse::<i32>() {
                Ok(val) => {
                    println!("mean: {}, median: {}, mode: {}", val, val, val);
                }
                Err(_) => println!("Encountered bad integer {}", val),
            },
            None => {
                println!("No input detected")
            }
        }
        return;
    }

    let mut sum = 0f32;
    let mut num_vec: Vec<i32> = Vec::new();
    let mut num_map: HashMap<i32, i32> = HashMap::new();
    for input_num in input_nums {
        match input_num.parse::<i32>() {
            Ok(val) => {
                sum += val as f32;
                num_vec.push(val);
                let count = num_map.entry(val).or_insert(0);
                *count += 1;
            }
            Err(_) => println!("Encountered bad integer {}", input_num),
        }
    }

    num_vec.sort();

    let mut mode = (0, 0);
    for (num, ocr) in num_map {
        if ocr >= mode.1 {
            mode = (num, ocr);
        }
    }

    let mean = sum / num_vec.len() as f32;

    let median: i32;
    if num_vec.len() % 2 == 1 {
        median = *num_vec
            .get((num_vec.len() / 2) + (num_vec.len() % 2))
            .unwrap();
    } else {
        let m = num_vec.len() / 2;
        median = (num_vec.get(m - 1).unwrap() + num_vec.get(m).unwrap()) / 2;
    }

    println!("mean: {}, median: {}, mode: {}", mean, median, mode.0);
}
