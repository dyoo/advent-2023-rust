use std::collections::HashSet;

fn part_1(s: &str) -> i32 {
    s.lines().filter_map(count_wins).sum()
}

fn count_wins(s: &str) -> Option<i32> {
    let (lhs, rhs) = s.split_once("|")?;
    let (_, winning) = lhs.split_once(":")?;
    let winning = winning
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<HashSet<i32>>();
    let scores = rhs.split_whitespace().filter_map(|s| s.parse::<i32>().ok());
    let winning_count = scores.filter(|s| winning.contains(s)).count() as u32;

    if winning_count == 0 {
        Some(0)
    } else {
        Some(2i32.pow(winning_count - 1))
    }
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

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("input");
    println!("Part 1: {}", part_1(&input));
}
