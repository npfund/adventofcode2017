use std::fs::File;
use std::io::Read;

fn main() {
    part1();
}

fn part1() {
    let zero: u8 = 48;
    let input = File::open("input.txt").expect("File not found!");

    let original_numbers = input.bytes()
        .map(|x| x.unwrap())
        .filter(|x| *x > zero)
        .map(|x| (x - zero) as u32)
        .peekable()
        .collect::<Vec<u32>>();

    let mut numbers = Vec::new();
    let mut num_iter = original_numbers.clone().into_iter().peekable();
    while let Some(number) = num_iter.next() {
        if let Some(next) = num_iter.peek() {
            if number == *next {
                numbers.push(number);
            }
        }
    }

    if original_numbers.first().unwrap() == original_numbers.last().unwrap() {
        numbers.push(*original_numbers.first().unwrap())
    }

    println!("{}", numbers.into_iter().sum::<u32>());
}
