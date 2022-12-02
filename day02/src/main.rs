fn score_round(r: &(u8, u8)) -> u32 {
    let mut score = 0;
    score += r.1 as u32 + 1;
    score += if r.0 == r.1 {
        3
    } else if (r.0 + 1)%3 == r.1 {
        6
    } else {
        0
    };
    score
}

fn main() {
    let rounds: Vec<_> = include_str!("../input").trim().as_bytes().split(|n| *n == b'\n').map(
        |line| (line[0]-b'A', line[2]-b'X')
    ).collect();

    // part 1
    let mut score: u32 = 0;
    for r in &rounds {
        score += score_round(r);
    }
    println!("part1: {}", score);

    let mut score: u32 = 0;
    for r in &rounds {
        let my_play = (r.0 + match r.1 {
            0 => 2,
            1 => 0,
            2 => 1,
            _ => unimplemented!(),
        }) % 3;
        score += score_round(&(r.0, my_play));
    }
    println!("part 2: {}", score);
}
