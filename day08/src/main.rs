use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::i32::MIN;

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));
    let lines = input.lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    println!("Maximum value after execution is {:?}", part1(&lines));
    println!("Maximum value during execution is {:?}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i32
{
    let mut registers: HashMap<&str, i32> = HashMap::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let register = parts[0];
        let sign = match parts[1] {
            "inc" => 1,
            "dec" => -1,
            _ => panic!("!"),
        };
        let value = parts[2].parse::<i32>().unwrap();
        //parts[3] is always if
        let source = parts[4];
        let operand = parts[6].parse::<i32>().unwrap();

        let source_value = *registers.entry(source).or_insert(0);
        let current_value = registers.entry(register).or_insert(0);
        let truth = match parts[5] {
            "<" => source_value < operand,
            ">" => source_value > operand,
            "<=" => source_value <= operand,
            ">=" => source_value >= operand,
            "==" => source_value == operand,
            "!=" => source_value != operand,
            _ => panic!("!!"),
        };

        if truth {
            *current_value += sign * value;
        }
    }

    let max_entry = registers.iter()
        .max_by(|l, r| l.1.cmp(r.1))
        .unwrap();

    return *max_entry.1;
}

fn part2(lines: &Vec<String>) -> i32
{
    let mut max: i32 = MIN;
    let mut registers: HashMap<&str, i32> = HashMap::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let register = parts[0];
        let sign = match parts[1] {
            "inc" => 1,
            "dec" => -1,
            _ => panic!("!"),
        };
        let value = parts[2].parse::<i32>().unwrap();
        //parts[3] is always if
        let source = parts[4];
        let operand = parts[6].parse::<i32>().unwrap();

        let source_value = *registers.entry(source).or_insert(0);
        let current_value = registers.entry(register).or_insert(0);
        let truth = match parts[5] {
            "<" => source_value < operand,
            ">" => source_value > operand,
            "<=" => source_value <= operand,
            ">=" => source_value >= operand,
            "==" => source_value == operand,
            "!=" => source_value != operand,
            _ => panic!("!!"),
        };

        if truth {
            *current_value += sign * value;
        }

        if *current_value > max {
            max = *current_value;
        }
    }

    return max;
}
