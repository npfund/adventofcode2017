use std::fs::File;
use std::io::Read;

fn main() {
    let zero: u8 = 48;
    let input = File::open("input.txt").expect("File not found!");

    let original_numbers = input.bytes()
        .map(|x| x.unwrap())
        .filter(|x| *x > zero)
        .map(|x| (x - zero) as u32)
        .peekable()
        .collect::<Vec<u32>>();

    println!("{}", part1(&original_numbers));
    println!("{}", part2(&original_numbers));
}

fn part1(original_numbers: &Vec<u32>) -> u32{
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

    return numbers.into_iter().sum::<u32>();
}

fn part2(original_numbers: &Vec<u32>) -> u32 {
    let length = original_numbers.len();


    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let vec1 = vec!(1, 1, 2, 2);
        assert_eq!(3, part1(&vec1));

        let vec2 = vec!(1, 1, 1, 1);
        assert_eq!(4, part1(&vec2));

        let vec3 = vec!(1, 2, 3, 4);
        assert_eq!(0, part1(&vec3));

        let vec4 = vec!(9, 1, 2, 1, 2, 1, 2, 9);
        assert_eq!(9, part1(&vec4));
    }

    #[test]
    fn part2_test() {
        let vec1 = vec!(1, 2, 1, 2);
        assert_eq!(6, part2(&vec1));

        let vec2 = vec!(1, 2, 2, 1);
        assert_eq!(0, part2(&vec2));

        let vec3 = vec!(1, 2, 3, 4, 2, 5);
        assert_eq!(4, part2(&vec3));

        let vec4 = vec!(1, 2, 3, 1, 2, 3);
        assert_eq!(12, part2(&vec4));

        let vec5 = vec!(1, 2, 1, 3, 1, 4, 1, 5);
        assert_eq!(4, part2(&vec5));
    }
}
