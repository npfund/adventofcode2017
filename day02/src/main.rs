use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));

    let spreadsheet = input.lines()
        .map(|x| x.unwrap()
            .split_whitespace()
            .map(|y| y.parse::<u32>().unwrap())
            .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    println!("{}", part1(&spreadsheet));
}

fn part1(spreadsheet: &Vec<Vec<u32>>) -> u32 {
    return spreadsheet.iter()
        .map(|x| x.iter().max().unwrap() - x.iter().min().unwrap())
        .sum::<u32>();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn part1_test() {
        let spreadsheet: Vec<Vec<u32>> = vec!(vec!(5, 1, 9, 5), vec!(7, 5, 3), vec!(2, 4, 6, 8));
        assert_eq!(18, part1(&spreadsheet));
    }
}
