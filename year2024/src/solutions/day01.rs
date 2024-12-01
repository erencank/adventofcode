use std::ops::Mul;

fn part_a(mut left: Vec<u64>, mut right: Vec<u64>) -> u64 {
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn part_b(left: &Vec<u64>, right: &Vec<u64>) -> u64 {
    left.iter()
        .map(|n| n.mul(right.iter().filter(|x| n == *x).count() as u64))
        .sum()
}

fn parse_lines(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip()
}

pub fn day01(input: &str) -> (String, String) {
    let (left, right) = parse_lines(input);
    (
        part_a(left.clone(), right.clone()).to_string(),
        part_b(&left, &right).to_string(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use year2024::read_file;

    #[test]
    fn test_example_parse_lines() {
        let input = read_file("examples", 1);
        let (left, right) = parse_lines(&input);
        assert_eq!(left, [3, 4, 2, 1, 3, 3]);
        assert_eq!(right, [4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 1);
        let (left, right) = parse_lines(&input);
        let diffs = part_a(left, right);
        assert_eq!(diffs, 11);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 1);
        let (left, right) = parse_lines(&input);
        let diffs = part_b(&left, &right);
        println!("{diffs}");
    }
}
