use std::collections::{VecDeque, HashMap};

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

struct Map {
    data: Vec<u8>,
    height: usize,
    width: usize,
}

impl Map {
    fn get(&self, (x, y): (i32,i32)) -> Option<u8> {
        if (0..self.width as i32).contains(&x) && (0..self.height as i32).contains(&y) {
            Some(self.data[x as usize + y as usize * self.width])
        } else {
            None
        }
    }
}

fn parse(input: &str) -> (Map, (i32, i32), (i32, i32)) {
    let mut height = 0;
    let mut data = vec![];
    let mut start = None;
    let mut end = None;
    for line in input.lines() {
        data.extend(line.bytes().enumerate().map(|(pos, ch)| match ch {
            b'S' => {
                start = Some((pos as i32, height as i32));
                b'a'
            }
            b'E' => {
                end = Some((pos as i32, height as i32));
                b'z'
            }
            ch => ch,
        }));
        height += 1;
    }
    let width = data.len() / height;
    (
        Map {
            data,
            height,
            width,
        },
        start.unwrap(),
        end.unwrap(),
    )
}

fn part1(input: &str) -> i32 {
    let (map, start, end) = parse(input);

    let mut best = HashMap::new();
    best.insert(start, 0);

    let mut work = VecDeque::new();
    work.push_back((start,0));

    while let Some((pt, cost)) = work.pop_front() {
        let steps = [
            (pt.0+1, pt.1),
            (pt.0-1, pt.1),
            (pt.0, pt.1+1),
            (pt.0, pt.1-1),
        ];
        let height = map.get(pt).unwrap();
        let nextcost = cost + 1;
        for nextpt in steps {
            match map.get(nextpt) {
                Some(h) if h <= height+1 => {
                    if best.get(&nextpt).map_or(true, |bestcost| nextcost < *bestcost) {
                        best.insert(nextpt, nextcost);
                        work.push_back((nextpt, nextcost));
                    }
                },
                None => (), // off the map
                _ => (), // too tall
            }
        }
    }

    *best.get(&end).unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../test1")), 31);
}


fn part2(input: &str) -> i32 {
    let (map, _start, end) = parse(input);

    let mut best = HashMap::new();
    best.insert(end, 0);

    let mut work = VecDeque::new();
    work.push_back((end,0));

    while let Some((pt, cost)) = work.pop_front() {
        let steps = [
            (pt.0+1, pt.1),
            (pt.0-1, pt.1),
            (pt.0, pt.1+1),
            (pt.0, pt.1-1),
        ];
        let height = map.get(pt).unwrap();
        let nextcost = cost + 1;
        for nextpt in steps {
            match map.get(nextpt) {
                Some(h) if h >= height - 1 => {
                    if best.get(&nextpt).map_or(true, |bestcost| nextcost < *bestcost) {
                        best.insert(nextpt, nextcost);
                        work.push_back((nextpt, nextcost));
                    }
                },
                None => (), // off the map
                _ => (), // too tall
            }
        }
    }

    best.iter().filter_map(|(pt, cost)| {
        if let Some(h) = map.get(*pt) {
            if h == b'a' {
                return Some(*cost)
            }
        }
        None
    }).min().unwrap()
}