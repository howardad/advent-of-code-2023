use std::fs;

fn main() {
    let text = fs::read_to_string("input.txt").expect("Failed to read file");
    let lines = text.split('\n').collect::<Vec<&str>>();
    let mut numbers: Vec<u32> = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let first = line.chars().find(|&c| c.is_ascii_digit()).unwrap().to_digit(10).unwrap();
        let second = line.chars().rev().find(|&c| c.is_ascii_digit()).unwrap().to_digit(10).unwrap();
        let number = first * 10 + second;
        numbers.push(number);
    }

    let sum: u32 = numbers.iter().sum();
    println!("The sum is: {}", sum);
}

