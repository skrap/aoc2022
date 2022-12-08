fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> usize {
    input.as_bytes().windows(4).enumerate().filter(|(_idx, wind)| {
        wind[0] != wind[1] && wind[0] != wind[2] && wind[0] != wind[3] &&
        wind[1] != wind[2] && wind[1] != wind[3] &&
        wind[2] != wind[3]
    }).next().unwrap().0 + 4
}


fn part2(input: &str) -> usize {
    input.as_bytes().windows(14).enumerate().filter(|(_idx, wind)| {
        let mut tmp = [false;256];
        for &n in *wind {
            if tmp[n as usize] { return false; }
            tmp[n as usize] = true;
        }
        true
    }).next().unwrap().0 + 14
}

#[test]
fn test_part1() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(part1(input), 5);
}
