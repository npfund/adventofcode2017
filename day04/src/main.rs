use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));
    let lines = input.lines()
        .map(|x| x.unwrap()).collect::<Vec<String>>();


    println!("{} of {} are valid passphrases", part1(&lines), lines.len());
    println!("{} of {} are valid passphrases", part2(&lines), lines.len());
}

fn part1(lines: &Vec<String>) -> u32 {
    let mut counter: u32 = 0;
    for line in lines.iter() {
        let mut map = HashMap::new();

        for word in line.split_whitespace() {
            let entry = map.entry(word).or_insert(0);
            *entry += 1;
        }

        if map.keys().count() == line.split_whitespace().count() {
            counter += 1;
        }
    }

    return counter;
}

fn part2(lines: &Vec<String>) -> u32 {
    let mut counter: u32 = 0;
    for line in lines.iter() {
        let mut map = HashMap::new();

        for word in line.split_whitespace() {
            let mut chars = word.chars().collect::<Vec<char>>();
            chars.sort_unstable();
            let sorted = String::from_iter(chars.iter());
            let entry = map.entry(sorted).or_insert(0);
            *entry += 1;
        }

        if map.keys().count() == line.split_whitespace().count() {
            counter += 1;
        }
    }

    return counter;
}
