/// https://adventofcode.com/2023/day/1
use aho_corasick::AhoCorasick;

fn part_1(s: &str) -> i32 {
    s.lines()
        .map(|l| {
            let s = l.chars().filter(char::is_ascii_digit).collect::<String>();
            let first_digit = s.chars().next().unwrap_or('0');
            let last_digit = s.chars().last().unwrap_or('0');
            10 * (first_digit as i32 - '0' as i32) + (last_digit as i32 - '0' as i32)
        })
        .sum()
}

fn part_2(s: &str) -> i32 {
    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();

    s.lines()
        .map(|l| {
            let first = ac
                .find_iter(l)
                .next()
                .map(|m| m.pattern().as_i32() % 9 + 1)
                .unwrap_or(0);
            let last = ac
                .find_overlapping_iter(l)
                .last()
                .map(|m| m.pattern().as_i32() % 9 + 1)
                .unwrap_or(0);
            println!("{} {} {}", l, first, last);
            first * 10 + last
        })
        .sum()
}

#[test]
fn test_part_1() {
    assert_eq!(
        part_1(
            "\
	1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
	"
        ),
        142
    )
}

#[test]
fn test_part_2() {
    assert_eq!(
        part_2(
            "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
	"
        ),
        281
    )
}

#[test]
fn test_overlapping() {
    assert_eq!(part_2("twone"), 21);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}
