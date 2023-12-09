type Num = i64;

fn parse_history_line(line: &str) -> Vec<Num> {
    line.split_whitespace()
        .filter_map(|n| n.parse::<Num>().ok())
        .collect()
}

fn parse_histories(lines: &str) -> Vec<Vec<Num>> {
    lines.lines().map(parse_history_line).collect()
}

fn predict_last(vals: &[Num]) -> Option<Num> {
    let diffs = differences(vals);
    if all_same(diffs.iter().copied()) {
        Some(vals.last()? + diffs.last()?)
    } else {
        let next = predict_last(&diffs)?;
        Some(vals.last()? + next)
    }
}

fn differences(vals: &[Num]) -> Vec<Num> {
    vals.windows(2)
        .filter_map(|window| match window {
            [x, y] => Some(y - x),
            _ => None,
        })
        .collect()
}

fn all_same(vals: impl IntoIterator<Item = Num>) -> bool {
    let mut items = vals.into_iter();
    if let Some(first) = items.next() {
        items.all(|item| item == first)
    } else {
        true
    }
}

#[test]
fn test_all_same() {
    assert!(all_same([]));
    assert!(all_same([3]));
    assert!(all_same([3, 3]));

    assert!(!all_same([3, 4]));
}

#[test]
fn test_predict_last() {
    assert_eq!(predict_last(&[]), None);
    assert_eq!(predict_last(&[1]), None);
    assert_eq!(predict_last(&[0, 0]), Some(0));
    assert_eq!(predict_last(&[0, 2, 4, 6]), Some(8));
    assert_eq!(predict_last(&[3, 3, 5, 9, 15]), Some(23));
    assert_eq!(predict_last(&[1, 3, 6, 10, 15, 21]), Some(28));
}

fn predict_first(vals: &[Num]) -> Option<Num> {
    let diffs = differences(vals);
    if all_same(diffs.iter().copied()) {
        Some(vals.first()? - diffs.last()?)
    } else {
        let diff = predict_first(&diffs)?;
        Some(vals.first()? - diff)
    }
}

#[test]
fn test_predict_first() {
    assert_eq!(predict_first(&[]), None);
    assert_eq!(predict_first(&[1]), None);
    assert_eq!(predict_first(&[0, 0]), Some(0));
    assert_eq!(predict_first(&[2, 2, 2]), Some(2));
    assert_eq!(predict_first(&[0, 2, 4, 6]), Some(-2));
    assert_eq!(predict_first(&[3, 3, 5, 9, 15]), Some(5));
    assert_eq!(predict_first(&[10, 13, 16, 21, 30, 45]), Some(5));
}

fn part_1(s: &str) -> Num {
    let histories = parse_histories(s);
    histories
        .into_iter()
        .filter_map(|history| predict_last(&history))
        .sum()
}

fn part_2(s: &str) -> Num {
    let histories = parse_histories(s);
    histories
        .into_iter()
        .filter_map(|history| predict_first(&history))
        .sum()
}

#[test]
fn test_part_1() {
    let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    assert_eq!(part_1(input), 114);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("input file");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
