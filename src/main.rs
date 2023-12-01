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

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("part 1: {}", part_1(&input));
}
