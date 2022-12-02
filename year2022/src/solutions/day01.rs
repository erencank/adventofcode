fn part_a(input: &Vec<u64>) -> &u64 {
    input.iter().max().unwrap()
}

fn part_b(input: &Vec<u64>) -> u64 {
    input.iter().take(3).sum::<u64>()
}

fn parse_lines(input: &str) -> Vec<u64> {
    let mut cal_per_elf: Vec<u64> = input
        .split("\n\n")
        .map(|total_elf| {
            total_elf
                .split('\n')
                .map(|calories| calories.parse::<u64>().unwrap_or(0))
                .sum()
        })
        .collect();
    cal_per_elf.sort_by(|a, b| b.cmp(a));

    return cal_per_elf;
}

pub fn day01(input: &str) -> (String, String) {
    let lines = parse_lines(input);
    (part_a(&lines).to_string(), part_b(&lines).to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    use year2022::read_file;

    #[test]
    fn test_example_parse_lines() {
        let input = read_file("examples", 1);
        let cal_per_elf = parse_lines(&input);
        let parsed = [24000, 11000, 10000, 6000, 4000];
        assert_eq!(cal_per_elf, parsed);
    }
    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 1);
        let cal_per_elf = parse_lines(&input);

        let answer: u64 = 24000;
        assert_eq!(part_a(&cal_per_elf), &answer);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 1);
        let cal_per_elf = parse_lines(&input);

        let answer: u64 = 45000;
        assert_eq!(part_b(&cal_per_elf), answer);
    }
}
