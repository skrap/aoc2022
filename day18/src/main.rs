fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
}

type Pt = [u8;3];

fn parse(input: &str) -> Vec<Pt> {
    input.lines().map(
        |line| {
            let mut it = line.split(",").map(|n| n.parse::<u8>().unwrap());
            [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
        }
    ).collect()
}

fn part1(input: &str) -> usize {
    let pts = parse(input);
    
}
