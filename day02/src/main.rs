#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: i32,
    rounds: Vec<Round>,
}

#[derive(Debug, PartialEq, Eq)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

fn parse_game_line(s: &str) -> Option<Game> {
    let mut chunks = s.split(':');
    if let (Some(game_id), Some(round_chunks)) = (chunks.next(), chunks.next()) {
        let id = game_id.split_whitespace().last()?.parse::<i32>().ok()?;
        Some(Game {
            id,
            rounds: round_chunks.split(';').map(parse_round).collect(),
        })
    } else {
        None
    }
}

fn parse_round(s: &str) -> Round {
    s.split(',').fold(
        Round {
            red: 0,
            green: 0,
            blue: 0,
        },
        |r, chunk| {
            let mut chunk = chunk.split_whitespace();
            if let (Some(count), Some(color)) = (chunk.next(), chunk.next()) {
                let count = count.parse::<i32>().unwrap_or(0);
                match color {
                    "red" => Round { red: count, ..r },
                    "green" => Round { green: count, ..r },
                    "blue" => Round { blue: count, ..r },
                    _ => r,
                }
            } else {
                r
            }
        },
    )
}

#[test]
fn test_parse_game_line() {
    assert_eq!(parse_game_line(""), None);
    assert_eq!(
        parse_game_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
        Some(Game {
            id: 1,
            rounds: vec![
                Round {
                    blue: 3,
                    red: 4,
                    green: 0
                },
                Round {
                    red: 1,
                    green: 2,
                    blue: 6
                },
                Round {
                    green: 2,
                    red: 0,
                    blue: 0
                },
            ]
        })
    );
}

fn is_good_game(g: &Game) -> bool {
    g.rounds
        .iter()
        .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
}

fn part_1(s: &str) -> i32 {
    let games = s.split('\n').filter_map(parse_game_line);
    games.filter(is_good_game).map(|g| g.id).sum()
}

#[test]
fn test_part_1() {
    let input = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    assert_eq!(part_1(input), 8);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("input.txt");
    println!("Part 1: {}", part_1(&input));
}
