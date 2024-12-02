fn part_a(levels: &Vec<Vec<u64>>) -> u64 {
    levels.iter().filter(|level| check_level(level)).count() as u64
}

fn part_b(levels: &Vec<Vec<u64>>) -> u64 {
    let mut total_safe = 0;
    for level in levels {
        if check_level(&level) {
            total_safe += 1;
        } else {
            for i in 0..(level.len()) {
                let mut modified_level = level.to_vec();
                modified_level.remove(i);
                if check_level(&modified_level) {
                    total_safe += 1;
                    break;
                }
            }
        }
    }
    total_safe
}

fn check_level(level: &Vec<u64>) -> bool {
    let mut increasing = false;
    let mut decreasing = false;
    for window in level.windows(2) {
        let diff = window[1] as i64 - window[0] as i64;
        if diff == 0 || diff.abs() > 3 {
            return false;
        } else if diff > 0 {
            if decreasing {
                return false;
            }
            increasing = true;
        } else {
            if increasing {
                return false;
            }
            decreasing = true;
        }
    }
    return true;
}

fn parse_lines(input: &str) -> Vec<Vec<u64>> {
    input
        .lines() // Split input into lines
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect() // Collect into a Vec<u64>
        })
        .collect()
}

pub fn day02(input: &str) -> (String, String) {
    let levels = parse_lines(input);
    (part_a(&levels).to_string(), part_b(&levels).to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    use year2024::read_file;

    #[test]
    fn test_example_parse_lines() {
        let input = read_file("examples", 2);
        let levels = parse_lines(&input);
        assert_eq!(levels[0], [7, 6, 4, 2, 1]);
    }

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 2);
        let levels = parse_lines(&input);
        let result = part_a(&levels);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 2);
        let levels = parse_lines(&input);
        let result = part_b(&levels);
        assert_eq!(result, 4);
    }
}
