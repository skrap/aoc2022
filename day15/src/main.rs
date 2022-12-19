use std::{collections::HashSet, ops::RangeBounds};

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input, 2000000));
    dbg!(part2(input, 4000000));
}

struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    radius: i32,
}

impl Sensor {
    fn new(mut it: impl Iterator<Item = i32>) -> Sensor {
        let pos = (it.next().unwrap(), it.next().unwrap());
        let beacon = (it.next().unwrap(), it.next().unwrap());
        let radius = (pos.0 - beacon.0).abs() + (pos.1 - beacon.1).abs();
        Sensor {
            pos,
            beacon,
            radius,
        }
    }
}

fn parse(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| {
            let it = line.split(&['=', ',', ':']);
            Sensor::new(it.filter_map(|s| s.parse::<i32>().ok()))
        })
        .collect()
}

fn part1(input: &str, y: i32) -> i32 {
    let sensors = parse(input);
    count_y(&sensors, y)
}

fn count_y(sensors: &[Sensor], y: i32) -> i32 {
    let mut edges = vec![];
    #[derive(PartialEq, PartialOrd, Ord, Eq)]
    enum EdgeType {
        Start,
        Stop,
    }
    use EdgeType::*;
    for s in sensors {
        let len_x = s.radius - (s.pos.1 - y).abs();
        if len_x >= 0 {
            let begin_x = s.pos.0 - len_x;
            let end_x = s.pos.0 + len_x;
            edges.push((begin_x, Start));
            edges.push((end_x, Stop));
        }
    }
    edges.sort();

    let mut n = 0;
    let mut state = 0;
    let mut span_start_x = None;
    for (x, dstate) in edges {
        if state == 0 {
            assert!(dstate == Start);
            span_start_x = Some(x);
            state += 1;
        } else {
            // in a span
            state += match dstate {
                Start => 1,
                Stop => -1,
            };
            if state == 0 {
                // span is closed
                n += x - span_start_x.unwrap() + 1;
                span_start_x = None;
            }
        }
    }

    let beacons_at_y: HashSet<_> = sensors
        .iter()
        .map(|s| s.beacon)
        .filter(|b| b.1 == y)
        .collect();

    n - beacons_at_y.len() as i32
}

#[test]
fn test_input() {
    let input = include_str!("../test1");
    assert_eq!(part1(input, 10), 26);
    assert_eq!(part2(input, 20), 56000011);
}

fn part2(input: &str, max: i32) -> usize {
    let sensors = parse(input);
    let range = 0..=max; 
    for y in range.clone() {
        if let Some(pt) = find_gap(&sensors, y, range.clone()) {
            return 4000000*pt.0 as usize + pt.1 as usize;
        }
    }
    panic!()
}

fn find_gap(sensors: &[Sensor], y: i32, xrange: impl RangeBounds<i32>) -> Option<(i32,i32)> {
    let mut edges = vec![];
    #[derive(PartialEq, PartialOrd, Ord, Eq)]
    enum EdgeType {
        Start,
        Stop,
    }
    use EdgeType::*;
    for s in sensors {
        let len_x = s.radius - (s.pos.1 - y).abs();
        if len_x >= 0 {
            let begin_x = s.pos.0 - len_x;
            let end_x = s.pos.0 + len_x;
            edges.push((begin_x, Start));
            edges.push((end_x, Stop));
        }
    }
    edges.sort();

    let mut state = 0;
    let mut last_close = i32::MIN;
    for (x, dstate) in edges {
        if state == 0 {
            assert!(dstate == Start);
            state += 1;
            if xrange.contains(&(last_close+1)) && x != last_close+1 {
                return Some((last_close+1,y));
            }
        } else {
            // in a span
            state += match dstate {
                Start => 1,
                Stop => -1,
            };
            if state == 0 {
                // span is closed
                last_close = x;
            }
        }
    }

    None
}