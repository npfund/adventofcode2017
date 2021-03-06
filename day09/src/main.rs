use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = File::open("input.txt").expect("File not found!");

    let mut buffer = String::new();
    input.read_to_string(&mut buffer).expect("Unable to read from file");
    let chars = buffer.chars()
        .filter(|x| !x.is_whitespace())
        .collect::<Vec<char>>();

    println!("Total score is {}", part1(&chars));
    println!("There are {} non-cancelled garbage characters", part2(&chars));
}

fn part1(chars: &Vec<char>) -> u32
{
    let mut score: u32 = 0;
    let mut depth: u32 = 0;
    let mut in_garbage = false;

    let mut c_iter = chars.iter();

    while let Some(c) = c_iter.next() {
        if in_garbage {
            match *c {
                '>' => in_garbage = false,
                '!' => {c_iter.next();},
                _ => {},
            }
        } else {
            match *c {
                '{' => depth += 1,
                '}' => {
                    score += depth;
                    depth -= 1
                },
                '<' => in_garbage = true,
                _ => {},
            }
        }
    }

    return score;
}

fn part2(chars: &Vec<char>) -> u32
{
    let mut count: u32 = 0;
    let mut in_garbage = false;

    let mut c_iter = chars.iter();

    while let Some(c) = c_iter.next() {
        if in_garbage {
            match *c {
                '>' => in_garbage = false,
                '!' => {c_iter.next();},
                _ => count += 1,
            }
        } else {
            match *c {
                '<' => in_garbage = true,
                _ => {},
            }
        }
    }

    return count;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test()
    {
        let chars = vec!('{', '}');
        assert_eq!(1, part1(&chars));

        let chars2 = vec!('{', '{', '{', '}', '}', '}');
        assert_eq!(6, part1(&chars2));

        let chars3 = vec!('{', '{', '}', ',', '{', '}', '}');
        assert_eq!(5, part1(&chars3));

        let chars4 = vec!('{', '{', '{', '}', ',', '{', '}', ',', '{', '{', '}', '}', '}', '}');
        assert_eq!(16, part1(&chars4));

        let chars5 = vec!('{', '<', 'a', '>', ',', '<', 'a', '>', ',', '<', 'a', '>', ',', '<', 'a', '>', '}');
        assert_eq!(1, part1(&chars5));

        let chars6 = vec!('{', '{', '<', 'a', 'b', '>', '}', ',', '{', '<', 'a', 'b', '>', '}', ',', '{', '<', 'a', 'b', '>', '}', ',', '{', '<', 'a', 'b', '>', '}', '}');
        assert_eq!(9, part1(&chars6));

        let chars7 = vec!('{', '{', '<', '!', '!', '>', '}', ',', '{', '<', '!', '!', '>', '}', ',', '{', '<', '!', '!', '>', '}', ',', '{', '<', '!', '!', '>', '}', '}');
        assert_eq!(9, part1(&chars7));

        let chars8 = vec!('{', '{', '<', 'a', '!', '>', '}', ',', '{', '<', 'a', '!', '>', '}', ',', '{', '<', 'a', '!', '>', '}', ',', '{', '<', 'a', 'b', '>', '}', '}');
        assert_eq!(3, part1(&chars8));
    }

    #[test]
    fn part2_test()
    {
        let chars = vec!('<', '>');
        assert_eq!(0, part2(&chars));

        let chars2 = "<random characters>".chars().collect::<Vec<char>>();
        assert_eq!(17, part2(&chars2));

        let chars3 = "<<<<>".chars().collect::<Vec<char>>();
        assert_eq!(3, part2(&chars3));

        let chars4 = "<{!>}>".chars().collect::<Vec<char>>();
        assert_eq!(2, part2(&chars4));

        let chars5 = "<!!>".chars().collect::<Vec<char>>();
        assert_eq!(0, part2(&chars5));

        let chars6 = "<!!!>>".chars().collect::<Vec<char>>();
        assert_eq!(0, part2(&chars6));

        let chars7 = "<{o\"i!a,<{i<a>".chars().collect::<Vec<char>>();
        assert_eq!(10, part2(&chars7));
    }
}
