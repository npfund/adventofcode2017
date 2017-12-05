use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));
    let lines = input.lines()
        .map(|x| x.unwrap()).collect::<Vec<String>>();

    let mut counter = 0;
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

    println!("{} of {} are valid passphrases", counter, lines.len());
}
