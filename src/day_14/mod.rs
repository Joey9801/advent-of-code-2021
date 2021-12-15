use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PairId(usize);

#[derive(Debug)]
pub struct Input {
    pairs: Vec<String>,

    init_str: String,

    // Count of each pair, indexed by PairId
    init_state: Vec<u64>,

    additions: Vec<(PairId, PairId)>,
}

fn parse_input(raw: &str) -> Input {
    let mut pairs = raw
        .lines()
        .skip(2)
        .map(|line| line[..2].to_string())
        .collect::<HashSet<_>>();
    let pairs = pairs.drain().collect::<Vec<_>>();

    let pair_map = pairs
        .iter()
        .enumerate()
        .map(|(idx, pair_str)| (pair_str.to_string(), PairId(idx)))
        .collect::<HashMap<_, _>>();

    assert!(pairs
        .iter()
        .flat_map(|k| k.chars().map(|c| c.is_ascii_uppercase()))
        .all(|x| x));

    let init_str = raw.lines().next().unwrap();
    let mut init_state = vec![0; pairs.len()];
    for i in 0..(init_str.len() - 1) {
        let pair = pair_map
            .get(&init_str[i..(i + 2)])
            .expect("Init pair string not in map");

        init_state[pair.0] += 1;
    }

    let mut additions = vec![(PairId(0), PairId(0)); pairs.len()];
    for op_line in raw.lines().skip(2) {
        let source_pair = *pair_map.get(&op_line[..2]).unwrap();
        let first_char = &op_line[0..1];
        let second_char = &op_line[1..2];
        let new_char = &op_line[6..7];

        let first_dest = *pair_map
            .get(&format!("{}{}", first_char, new_char))
            .expect("Expect newly formed pair to be in map");
        let second_dest = *pair_map
            .get(&format!("{}{}", new_char, second_char))
            .expect("Expect newly formed pair to be in map");

        additions[source_pair.0] = (first_dest, second_dest);
    }

    Input {
        pairs,
        init_str: init_str.to_string(),
        init_state,
        additions,
    }
}

fn do_solve(input: &Input, rounds: u16) -> u64 {
    let mut a = input.init_state.clone();
    let mut b = input.init_state.clone();
    let pair_count = a.len();

    for iteration in 0..rounds {
        let (read, write) = if iteration % 2 == 0 {
            (&a, &mut b)
        } else {
            (&b, &mut a)
        };

        write.iter_mut().for_each(|c| *c = 0);

        for i in 0..pair_count {
            let (dest_1, dest_2) = input.additions[i];
            write[dest_1.0] += read[i];
            write[dest_2.0] += read[i];
        }
    }
    let final_state = if rounds % 2 == 0 { a } else { b };

    let mut histogram = [0; 26];

    for (pair_idx, count) in final_state.iter().enumerate() {
        let histo_index = (input.pairs[pair_idx].as_bytes()[1] - b'A') as usize;
        histogram[histo_index] += *count;
    }
    histogram[(input.init_str.as_bytes()[0] - b'A') as usize] += 1;

    histogram.sort();

    let smallest = *histogram.iter().filter(|x| **x != 0).next().unwrap();
    let largest = histogram[25];

    largest - smallest
}

fn part_1(input: &Input) -> u64 {
    do_solve(input, 10)
}

fn part_2(input: &Input) -> u64 {
    do_solve(input, 40)
}

impl_day!("2021", "14", "Extended Polymerization", Input, u64, u64);

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Input {
        parse_input(
            "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C",
        )
    }

    #[test]
    fn test_part_1_example() {
        let input = example_input();
        assert_eq!(part_1(&input), 1588);
    }

    #[test]
    fn test_part_2_example() {
        let input = example_input();
        assert_eq!(part_2(&input), 2188189693529);
    }
}
