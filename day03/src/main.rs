fn main() {
    let input: f32 = 277678.0;

    println!("So {} is {} steps away from the center", input, part1(input));
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
