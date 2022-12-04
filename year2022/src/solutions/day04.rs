fn part_a(input: &Vec<Vec<Vec<u32>>>) -> u32 {
    input.iter().map(|f| overlap(f, false)).sum::<u32>()
}

fn part_b(input: &Vec<Vec<Vec<u32>>>) -> u32 {
    input.iter().map(|f| overlap(f, true)).sum::<u32>()
}

fn overlap(input: &Vec<Vec<u32>>, partial_contain: bool) -> u32 {
    let index = if partial_contain { 0 } else { 1 };
    let (lsplit, rsplit) = (&input[0], &input[1]);
    if lsplit[0] <= rsplit[0] && lsplit[1] >= rsplit[index]
        || (rsplit[0] <= lsplit[0]) && (rsplit[1] >= lsplit[index])
    {
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
