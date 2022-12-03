fn part1(input: &str) -> u32 {
    let input: Vec<_> = input.trim().lines().collect();

    let mut prios = vec![];
    for line in input {
        let (s1, s2) = line.split_at(line.len()/2);
        let mut item = None;
        for ch1 in s1.chars() {
            if s2.chars().any(|ch2| ch2 == ch1) {
                item = Some(ch1);
                break;
            }
        }
        prios.push(to_prio(item.unwrap()));
    }

    prios.iter().sum()
}

fn to_prio(item: char) -> u32 {
    match item {
        a @ 'A'..='Z' => 27 + a as u32 - b'A' as u32,
        a @ 'a'..='z' => 1 + a as u32 - b'a' as u32,
        _ => unimplemented!(),
    }
}

fn part2(input: &str) -> u32 {
    let lines: Vec<_> = input.trim().lines().collect();
    let mut prios = vec![];
    for (_idx,group) in lines.chunks(3).enumerate(){
        for ch1 in group[0].chars() {
            if group[1].chars().any(|c| c == ch1) && group[2].chars().any(|c| c == ch1) {
                prios.push(to_prio(ch1));
                break; // needed because sometimes the badge is the duplicated item
            }
        }
    }
    prios.iter().sum()
}

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}
