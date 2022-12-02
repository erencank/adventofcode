fn part_a(input: &str) -> usize {
    input.split("\n").map(|round| calc_points(round, 0)).sum()
}

fn part_b(input: &str) -> usize {
    input.split("\n").map(|round| calc_points(round, 1)).sum()
}

fn calc_points(input: &str, part: usize) -> usize {
    let scorecard: Vec<[[usize; 3]; 3]> = vec![
        [[4, 8, 3], [1, 5, 9], [7, 2, 6]],
        [[3, 4, 8], [1, 5, 9], [2, 6, 7]],
    ];
    let v: Vec<usize> = input
        .split(" ")
        .into_iter()
        .map(|hand| convert_to_int(hand))
        .collect();

    scorecard[part][v[0]][v[1]]
}

fn convert_to_int(char: &str) -> usize {
    match char {
        "A" => return 0,
        "B" => return 1,
        "C" => return 2,
        "X" => return 0,
        "Y" => return 1,
        "Z" => return 2,
        _ => panic!("{}", char),
    }
}

pub fn day02(input: &str) -> (String, String) {
    (part_a(input).to_string(), part_b(input).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_calc_points_part_a() {
        assert_eq!(calc_points("A Y", 0), 8);
        assert_eq!(calc_points("B X", 0), 1);
        assert_eq!(calc_points("C Z", 0), 6);

        assert_eq!(calc_points("A Y", 1), 4);
        assert_eq!(calc_points("B X", 1), 1);
        assert_eq!(calc_points("C Z", 1), 7);
    }
}
