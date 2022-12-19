use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn parse(input: &str) -> HashMap<(i32,i32), u8> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let segments: Vec<_> = line.split(" -> ").map(|pt| {
            let (x,y) = pt.split_once(",").unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        }).collect();
        if segments.len() == 1 {
            map.insert(segments[0], b'#');
        } else {
            for s in segments.windows(2) {
                let (start, end) = (s[0], s[1]);
                map.insert(start, b'#');

                let dx = (s[1].0 - s[0].0).signum();
                let dy = (s[1].1 - s[0].1).signum();
                let mut pt = start;
                while pt != end {
                    pt.0 += dx;
                    pt.1 += dy;
                    map.insert(pt, b'#');
                }
            }
        }
    }

    map
}

fn part1(input: &str) -> usize {
    let mut map = parse(input);

    let maxy = map.iter().map(|(pt, _)| pt.1).max().unwrap();

    let mut grains = 0;

    'grain: loop {
        // drop a grain
        let (mut x, mut y) = (500,0);
        while y <= maxy {
            let options = [(x,y+1), (x-1,y+1), (x+1,y+1)];
            let next_pt = options.iter().filter(|pt| !map.contains_key(pt)).next();
            match next_pt {
                None => {
                    // grain comes to rest
                    grains += 1;
                    map.insert((x,y), b'o');
                    continue 'grain;
                }
                Some(pt) => (x,y) = *pt,
            }
        }
        break;
    }

    grains
}

#[test]
fn test_part1() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    assert_eq!(part1(input), 24);
    assert_eq!(part2(input), 93);
}


fn part2(input: &str) -> usize {
    let mut map = parse(input);

    let maxy = map.iter().map(|(pt, _)| pt.1).max().unwrap();

    let mut grains = 0;

    'grain: loop {
        // drop a grain
        let (mut x, mut y) = (500,0);
        loop {
            let options = [(x,y+1), (x-1,y+1), (x+1,y+1)];
            let next_pt = options.iter().filter(|pt| !map.contains_key(pt) && pt.1 < maxy+2).next();
            match next_pt {
                None => {
                    // grain comes to rest
                    grains += 1;
                    map.insert((x,y), b'o');
                    if y == 0 {
                        break 'grain;
                    }
                    continue 'grain;
                }
                Some(pt) => (x,y) = *pt,
            }
        }
    }

    grains
}