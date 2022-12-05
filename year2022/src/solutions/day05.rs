fn part_a(input: (Vec<Vec<char>>, Vec<Vec<usize>>)) -> String {
    let (mut crates, moves) = input;
    for move_ in moves {
        for _m in 0..move_[0] {
            let take_pile = {
                match crates[move_[1] - 1].pop() {
                    Some(crate_) => crate_,
                    None => {
                        panic!("AAAAAAAAAAAAAAA")
                    }
                }
            };
            crates[move_[2] - 1].push(take_pile);
        }
    }
    show_result(&crates)
}

fn part_b(input: (Vec<Vec<char>>, Vec<Vec<usize>>)) -> String {
    let (mut crates, moves) = input;
    println!("{:?}", crates);
    for move_ in moves {
        println!("Move: {:?}", move_);
        for amount in 0..move_[0] {
            let take_pile = {
                match crates[move_[1] - 1].pop() {
                    Some(crate_) => crate_,
                    None => {
                        panic!("AAAAAAAAAAAAAAA")
                    }
                }
            };
            let pile_len = crates[move_[2] - 1].len();

            crates[move_[2] - 1].insert(pile_len - amount, take_pile);
            println!("{:?}", crates);
        }
    }
    show_result(&crates)
}

fn parse_lines(input: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let mut iter = input.split("\n\n");
    let (crates, moves) = (iter.next().unwrap(), iter.next().unwrap());
    println!("{}", crates);

    let crate_piles = parse_crates(crates);
    let crate_moves = parse_moves(moves);

    (crate_piles, crate_moves)
}

fn parse_crates(crates: &str) -> Vec<Vec<char>> {
    let crates_per_level: Vec<Vec<&str>> = crates
        .lines()
        .map(|layer| {
            let mut section: Vec<&str> = Vec::new();
            let mut _layer = layer.clone();

            loop {
                match _layer.char_indices().nth(4) {
                    Some((offset, _)) => {
                        let (a, b) = _layer.split_at(offset);
                        section.push(a.trim());
                        _layer = b;
                    }
                    None => {
                        section.push(_layer.trim());

                        return section;
                    }
                }
            }
        })
        .collect();

    let layers = crates_per_level.len() - 1;
    let piles = crates_per_level[0].len();

    let mut crate_pile: Vec<Vec<char>> = Vec::with_capacity(piles);
    for _p in 0..piles {
        crate_pile.push(Vec::with_capacity(layers));
    }

    for layer in 0..layers {
        let row = crates_per_level[layer].clone();
        row.iter()
            .enumerate()
            .filter(|(_, val)| !val.is_empty())
            .for_each(|(i, val)| crate_pile[i].insert(0, val.chars().nth(1).unwrap().clone()));
    }
    crate_pile
}

fn parse_moves(moves: &str) -> Vec<Vec<usize>> {
    moves
        .lines()
        .map(|move_| {
            move_
                .split(" ")
                .skip(1)
                .step_by(2)
                .map(|digit| digit.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn show_result(crates: &Vec<Vec<char>>) -> String {
    let mut result: String = "".to_owned();
    crates
        .iter()
        .for_each(|pile| result.push_str(&pile[pile.len() - 1].to_string()));
    result
}

pub fn day05(input: &str) -> (String, String) {
    let lines = parse_lines(&input);
    (part_a(lines.clone()), part_b(lines.clone()))
}
#[cfg(test)]
mod tests {
    use super::*;
    use year2022::read_file;

    #[test]
    fn test_example_parse_lines() {
        let input = read_file("examples", 5);
        let lines = parse_lines(&input);
        let res = part_a(lines);
        println!("{}", res);
    }
    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 5);
        let lines = parse_lines(&input);
        let res = part_b(lines);
        println!("{}", res);
    }
}
