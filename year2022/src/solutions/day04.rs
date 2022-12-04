fn part_a(input: &Vec<Vec<Vec<u32>>>) -> u32 {
    input.iter().map(|f| calc_overlap2(f, 1)).sum::<u32>()
}

fn part_b(input: &Vec<Vec<Vec<u32>>>) -> u32 {
    input.iter().map(|f| calc_overlap2(f, 0)).sum::<u32>()
}

fn calc_overlap2(input: &Vec<Vec<u32>>, right_compare: usize) -> u32 {
    let lsplit = &input[0];
    let rsplit = &input[1];
    if lsplit[0] <= rsplit[0] && lsplit[1] >= rsplit[right_compare] {
        1
    } else if (rsplit[0] <= lsplit[0]) && (rsplit[1] >= lsplit[right_compare]) {
        1
    } else {
        0
    }
}

fn parse_lines(input: &str) -> Vec<Vec<Vec<u32>>> {
    input
        .lines()
        .into_iter()
        .map(|tasks| {
            tasks
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|areas| {
                    areas
                        .split("-")
                        .map(|val| val.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn day04(input: &str) -> (String, String) {
    let lines = parse_lines(&input);
    (part_a(&lines).to_string(), part_b(&lines).to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    use year2022::read_file;

    #[test]
    fn test_example_parse_lines() {
        let input = read_file("examples", 4);
        let lines = parse_lines(&input);
        let overlaps = part_a(&lines);
        println!("{}", overlaps);
        assert_eq!(overlaps, 2);
    }
    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 4);
        let lines = parse_lines(&input);
        let overlaps = part_b(&lines);
        println!("{}", overlaps);
        assert_eq!(overlaps, 4);
    }
}
