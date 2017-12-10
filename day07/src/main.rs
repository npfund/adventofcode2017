use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

extern crate regex;

use regex::Regex;

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));
    let lines = input.lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    println!("{} is the root process", part1(lines));
}

fn part1(lines: Vec<String>) -> String
{
    let mut names = HashSet::new();
    let mut held_names: HashSet<String> = HashSet::new();

    let re = Regex::new(r"^([a-z]+) \(\d+\)(?: -> )?([a-z, ]+)?$").unwrap();
    for line in lines {
        let captures = re.captures(&line)
            .unwrap()
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap().as_str())
            .collect::<Vec<&str>>();
        names.insert(captures[1].to_string());

        if let Some(second) = captures.get(2) {
            for name in second.split(", ") {
                held_names.insert(name.to_string());
            }
        }
    }

    return names.difference(&held_names).last().unwrap().clone();
}
