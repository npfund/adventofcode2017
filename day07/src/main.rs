use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;

extern crate regex;
extern crate itertools;

use regex::Regex;
use itertools::Itertools;

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));
    let lines = input.lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    println!("{} is the root process", part1(&lines));
    println!("{}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> String
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

fn part2(lines: &Vec<String>) -> String
{
    let mut names = HashMap::new();
    let mut held_names: HashMap<String, Vec<String>> = HashMap::new();

    let re = Regex::new(r"^([a-z]+) \((\d+)\)(?: -> )?([a-z, ]+)?$").unwrap();
    for line in lines {
        let captures = re.captures(&line)
            .unwrap()
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap().as_str())
            .collect::<Vec<&str>>();

        let program = captures[1].to_string();
        names.insert(program.clone(), captures[2].parse::<usize>().unwrap());

        if let Some(second) = captures.get(3) {
            held_names.insert(program, second.split(", ").map(|x| x.to_string()).collect());
        } else {
            held_names.insert(program, Vec::new());
        }
    }

    println!("{:?}", weigh(&names, &held_names, "yruivis"));
    return String::new();
}

fn weigh<'a>(weights: &HashMap<String, usize>, names: &HashMap<String, Vec<String>>, root: &'a str) -> usize
{
    let children = names.get(root).unwrap();
    if children.len() > 0 {
        let child_weights = children.iter().map(|x| return weigh(weights, names, x)).collect::<Vec<usize>>();
        let child_sum = child_weights.iter().sum::<usize>();
            println!("{:?}", root);
            println!("{:?}", children);
            println!("{:?}", child_weights);
        for child in children {
            println!("{} -> {}", child, weights.get(child).unwrap());
        }
        return *weights.get(root).unwrap() + child_sum;
    } else {
        return *weights.get(root).unwrap();
    }
}

struct Program {
    name: String,
    weight: usize,
    children: Vec<Program>
}
