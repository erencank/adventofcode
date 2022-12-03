use std::collections::HashSet;

fn part_a(input: &str) -> u32 {
    let items: Vec<char> = input
        .lines()
        .map(|x| intersection(&x[..x.len() / 2], &x[x.len() / 2..])[0])
        .collect();
    calc_priority(&items)
}
fn part_b(input: &str) -> u32 {
    let backpacks: Vec<&str> = input.lines().collect();
    let mut items: Vec<char> = Vec::new();
    for i in (0..backpacks.len()).step_by(3) {
        let sets = &backpacks[i..i + 3];
        let vec = intersection(sets[0], sets[1]);
        let intersect = intersection(&String::from_iter(vec), sets[2]);
        items.push(intersect[0]);
    }
    calc_priority(&items)
}

fn intersection(left: &str, right: &str) -> Vec<char> {
    let set1: HashSet<char> = left.chars().collect();
    let set2: HashSet<char> = right.chars().collect();
    let a = set1.intersection(&set2);
    let res: Vec<char> = a.map(|c| c.clone()).collect();
    return res;
}

fn calc_priority(items: &Vec<char>) -> u32 {
    items
        .iter()
        .map(|prio| {
            if prio.is_uppercase() {
                prio.clone() as u32 - 38
            } else {
                prio.clone() as u32 - 96
            }
        })
        .sum()
}

pub fn day03(input: &str) -> (String, String) {
    (part_a(input).to_string(), part_b(input).to_string())
}

#[cfg(test)]
mod tests {

    use super::*;
    use year2022::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 3);
        assert_eq!(part_a(&input), 157);
    }
    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 3);
        assert_eq!(part_b(&input), 70);
    }

    #[test]
    fn test_example_intersect() {
        let sets = [
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        let vec = intersection(sets[0], sets[1]);
        println!("{:?}", vec);
        let left = String::from_iter(vec);
        println!("{}", left);
        let intersect = intersection(&left, sets[2]);
        println!("{:?}", intersect);
        assert_eq!(calc_priority(&vec!['r']), 18);
        println!("{}", calc_priority(&vec!['r']));

        assert_eq!(calc_priority(&vec!['Z']), 52);
        println!("{}", calc_priority(&vec!['Z']));
    }
}
