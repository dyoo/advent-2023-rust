use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: i32,
    winning: HashSet<i32>,
    holdings: Vec<i32>,
}

impl Card {
    fn winning_count(&self) -> i32 {
        self.holdings
            .iter()
            .filter(|h| self.winning.contains(h))
            .count() as i32
    }
}

fn parse_cards(s: &str) -> Vec<Card> {
    s.lines()
        .filter_map(|line| {
            let (lhs, rhs) = line.split_once('|')?;
            let (_, winning) = lhs.split_once(':')?;
            let winning = winning
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<HashSet<i32>>();
            let holdings = rhs
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            Some(Card {
                id: 0,
                winning,
                holdings,
            })
        })
        .collect()
}

fn part_1(s: &str) -> i32 {
    let cards = parse_cards(s);
    cards
        .iter()
        .map(|card| {
            let winning_count = card.winning_count();
            if winning_count == 0 {
                0
            } else {
                2i32.pow(winning_count as u32 - 1)
            }
        })
        .sum::<i32>()
}

fn part_2(s: &str) -> i32 {
    let cards = parse_cards(s);
    let mut inventory: Vec<i32> = vec![1; cards.len()];
    for i in 0..inventory.len() {
        let card = &cards[i];
        let multiplier = inventory[i];
        for j in 1..=card.winning_count() as usize {
            inventory[i + j] += multiplier;
        }
    }
    inventory.into_iter().sum()
}

#[test]
fn test_part_1() {
    let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    assert_eq!(part_1(input), 13);
}

#[test]
fn test_part_2() {
    let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    assert_eq!(part_2(input), 30);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("input");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
