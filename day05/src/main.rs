// https://adventofcode.com/2023/day/5
#[derive(Debug, PartialEq, Eq)]
struct ProblemStatement {
    seeds: Vec<u32>,
    maps: Vec<PlantingMap>,
}

#[derive(Debug, PartialEq, Eq)]
struct PlantingMap {
    source: String,
    dest: String,
    ranges: Vec<PlantingRange>,
}

#[derive(Debug, PartialEq, Eq)]
struct PlantingRange {
    dest_start: u32,
    source_start: u32,
    length: u32,
}

fn parse_problem_statement(input: &str) -> Option<ProblemStatement> {
    let mut chunks = input.trim().split("\n\n");
    let seeds = chunks
        .next()?
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().ok())
        .collect::<Option<Vec<u32>>>()?;
    let maps =
	chunks.map(parse_planting_map).collect::<Option<Vec<PlantingMap>>>()?;
    Some(ProblemStatement { seeds, maps })
}

fn parse_planting_map(s: &str) -> Option<PlantingMap> {
    let mut lines = s.split("\n");
    let mut title = lines.next()?.split(" ").next()?.split("-");
    let (source, _, dest) = (title.next()?, title.next()?, title.next()?);
    let ranges = lines.map(parse_planting_range).collect::<Option<Vec<PlantingRange>>>()?;
    Some(PlantingMap {source: source.into(), dest: dest.into(), ranges})
}

fn parse_planting_range(s: &str) -> Option<PlantingRange> {
    let mut vals = s.split_whitespace();
    Some(PlantingRange {
	    dest_start: vals.next()?.parse::<u32>().ok()?,
	    source_start: vals.next()?.parse::<u32>().ok()?,
	    length: vals.next()?.parse::<u32>().ok()?
	})
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let p = parse_problem_statement(&input);
    println!("{:#?}", p);
}
