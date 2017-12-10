use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let mut input = File::open("input.txt").expect("File not found!");
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).expect("Could not read file");

    let banks = buffer.split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    println!("It takes {} cycles to detect a loop", part1(&banks));
    println!("The loop is {} cycles long", part2(&banks));
}

fn part1(banks: &Vec<u32>) -> u32
{
    let len = banks.len();
    let mut states = HashSet::new();

    let mut current_state = banks.clone();
    let mut cycle_count = 0;
    while !states.contains(&current_state) {
        states.insert(current_state.clone());

        let current_state_clone = current_state.clone();
        let max = current_state_clone.iter()
            .enumerate()
            .rev()
            .max_by(|l, r| l.1.cmp(r.1))
            .unwrap();

        current_state[max.0] = 0;
        for i in 1..(max.1 + 1) as usize {
            current_state[(max.0 + i) % len] += 1;
        }

        cycle_count += 1;
    }

    return cycle_count;
}

fn part2(banks: &Vec<u32>) -> u32
{
    let len = banks.len();
    let mut states = HashMap::new();

    let mut current_state = banks.clone();
    let mut cycle_count: u32 = 0;
    while !states.contains_key(&current_state) {
        states.insert(current_state.clone(), cycle_count);

        let current_state_clone = current_state.clone();
        let max = current_state_clone.iter()
            .enumerate()
            .rev()
            .max_by(|l, r| l.1.cmp(r.1))
            .unwrap();

        current_state[max.0] = 0;
        for i in 1..(max.1 + 1) as usize {
            current_state[(max.0 + i) % len] += 1;
        }

        cycle_count += 1;
    }

    return cycle_count - states.get(&current_state).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test()
    {
        let banks: Vec<u32> = vec!(0, 2, 7, 0);
        assert_eq!(5, part1(&banks));
    }

    #[test]
    fn part2_test()
    {
        let banks: Vec<u32> = vec!(0, 2, 7, 0);
        assert_eq!(4, part2(&banks));
    }
}
