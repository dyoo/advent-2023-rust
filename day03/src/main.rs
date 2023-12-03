use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq)]
struct Number {
    value: i32,
    y: i32,
    xs: RangeInclusive<i32>,
}

impl Number {
    fn is_adjacent_to(&self, y: i32, x: i32) -> bool {
        self.y.abs_diff(y) <= 1
            && self
                .xs
                .clone()
                .any(|x_in_range| x_in_range.abs_diff(x) <= 1)
    }
}

fn find_all_numbers(s: &str) -> Vec<Number> {
    fn build_number(digits: &[(usize, char)], y: usize) -> Number {
        Number {
            value: digits
                .iter()
                .cloned()
                .fold(0, |acc, (_, ch)| acc * 10 + (ch as i32 - '0' as i32)),
            y: y as i32,
            xs: (digits.first().unwrap().0 as i32)..=(digits.last().unwrap().0 as i32),
        }
    }

    let mut result = vec![];
    for (y, line) in s.lines().enumerate() {
        let mut digits = vec![];
        for (x, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                digits.push((x, ch));
            } else if !digits.is_empty() {
                result.push(build_number(&digits, y));
                digits = vec![];
            }
        }
        if !digits.is_empty() {
            result.push(build_number(&digits, y));
        }
    }
    result
}

#[test]
fn test_find_all_numbers() {
    let schematic = "\
467..
...114..
";
    assert_eq!(
        find_all_numbers(schematic),
        vec![
            Number {
                value: 467,
                y: 0,
                xs: 0..=2,
            },
            Number {
                value: 114,
                y: 1,
                xs: 3..=5,
            }
        ]
    );
}

#[test]
fn test_is_adjacent() {
    let n = Number {
        value: 467,
        y: 0,
        xs: 0..=2,
    };
    assert!(n.is_adjacent_to(1, 0));
    assert!(n.is_adjacent_to(1, 1));
    assert!(n.is_adjacent_to(1, 2));
    assert!(n.is_adjacent_to(1, 3));
    assert!(!n.is_adjacent_to(1, 4));
}

fn find_all_symbols(s: &str) -> Vec<(char, i32, i32)> {
    let mut symbols = vec![];
    for (y, line) in s.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' && !ch.is_ascii_digit() {
                symbols.push((ch, y as i32, x as i32));
            }
        }
    }
    symbols
}

fn part_1(s: &str) -> i32 {
    let nums = find_all_numbers(s);
    let symbols = find_all_symbols(s);
    nums.into_iter()
        .filter(|num| symbols.iter().any(|&sym| num.is_adjacent_to(sym.1, sym.2)))
        .map(|num| num.value)
        .sum()
}

#[test]
fn test_part_1() {
    let input ="\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    assert_eq!(part_1(input),
	       4361);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("part 1: {}", part_1(&input));
}
