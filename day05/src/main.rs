use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));
    let jumps = input.lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("It takes {} steps to exit the list", part1(&jumps));
}

fn part1(jumps: &Vec<i32>) -> u32 {
    let mut list = jumps.clone();
    let mut steps: u32 = 0;
    let mut current: usize = 0;
    loop {
        match list.get_mut(current) {
            Some(jump) => {
                current = ((current as i32) + *jump) as usize;
                steps += 1;
                *jump += 1;
            },
            None => break,
        }
    }

    return steps;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let jumps = vec!(0, 3, 0, 1, -3);
        assert_eq!(5, part1(&jumps));
    }
}
