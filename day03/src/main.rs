fn main() {
    let input: f32 = 277678.0;

    println!("So {} is {} steps away from the center", input, part1(input));
    println!("{} is the first value to exceed the input", part2(input as u64));
}

fn part1(input: f32) -> f32 {
    let root = input.sqrt();
    let mut square = root.ceil();
    if square % 2.0 == 0.0 {
        square += 1.0;
    }

    let ring = (square / 2.0).floor();
    let corner = square.powi(2);
    let corner_distance = square - 1.0;
    let mut distance_from_corner = (input - corner).abs() % (ring + 1.0);
    if distance_from_corner.is_nan() {
        distance_from_corner = 0.0;
    }

    println!("sqrt({}) is {}", input, root);
    println!("Therefore {} is on the ring that ends with square {}", input, square);
    println!("Square {} terminates ring number {}", square, ring);
    println!("Min distance is {}, max distance at corner is {}", ring, corner_distance);
    println!("{} is {} steps away from corner", input, distance_from_corner);

    return corner_distance - distance_from_corner;
}

fn part2(input: u64) -> u64 {
    let mut matrix = [[0 as u64; 1001]; 1001];

    let mut output = 0;
    let mut ring = 1;

    let mut x = 500;
    let mut y = 500;

    matrix[y][x] = 1;
    x += 1;

    'outer: while input > output {
        for _ in 0..((2 * ring) - 1) {
            matrix[y][x] = matrix[y - 1][x] + matrix[y + 1][x] + matrix[y][x + 1] + matrix[y][x - 1]
                + matrix[y - 1][x - 1] + matrix[y + 1][x + 1] + matrix[y - 1][x + 1] + matrix[y + 1][x - 1];
            output = matrix[y][x];
            if output > input {
                break 'outer;
            }
            y -= 1;
        }

        for _ in 0..(2 * ring) {
            matrix[y][x] = matrix[y - 1][x] + matrix[y + 1][x] + matrix[y][x + 1] + matrix[y][x - 1]
                + matrix[y - 1][x - 1] + matrix[y + 1][x + 1] + matrix[y - 1][x + 1] + matrix[y + 1][x - 1];
            output = matrix[y][x];
            if output > input {
                break 'outer;
            }
            x -= 1;
        }

        for _ in 0..(2 * ring) {
            matrix[y][x] = matrix[y - 1][x] + matrix[y + 1][x] + matrix[y][x + 1] + matrix[y][x - 1]
                + matrix[y - 1][x - 1] + matrix[y + 1][x + 1] + matrix[y - 1][x + 1] + matrix[y + 1][x - 1];
            output = matrix[y][x];
            if output > input {
                break 'outer;
            }
            y += 1;
        }

        for _ in 0..(2 * ring) {
            matrix[y][x] = matrix[y - 1][x] + matrix[y + 1][x] + matrix[y][x + 1] + matrix[y][x - 1]
                + matrix[y - 1][x - 1] + matrix[y + 1][x + 1] + matrix[y - 1][x + 1] + matrix[y + 1][x - 1];
            output = matrix[y][x];
            if output > input {
                break 'outer;
            }
            x += 1;
        }

        matrix[y][x] = matrix[y - 1][x] + matrix[y + 1][x] + matrix[y][x + 1] + matrix[y][x - 1]
            + matrix[y - 1][x - 1] + matrix[y + 1][x + 1] + matrix[y - 1][x + 1] + matrix[y + 1][x - 1];
        output = matrix[y][x];
        x += 1;

        ring += 1;
    }

    return output;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(0.0, part1(1.0));
        assert_eq!(3.0, part1(12.0));
        assert_eq!(2.0, part1(23.0));
//        assert_eq!(31.0, part1(1024.0));
    }
}
