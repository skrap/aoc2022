use std::{collections::HashSet, iter::repeat};

fn main() {
    let input = include_bytes!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

struct Map {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y * self.width + x]
    }
}

fn parse(input: &[u8]) -> Map {
    let width = input.split(|d| *d == b'\n').next().unwrap().len();
    let data: Vec<_> = input
        .iter()
        .filter_map(|d| match *d {
            b'0'..=b'9' => Some(d - b'0'),
            b'\n' => None,
            _ => unimplemented!(),
        })
        .collect();
    let height = data.len() / width;
    Map {
        data,
        width,
        height,
    }
}

fn part1(input: &[u8]) -> usize {
    let map = parse(input);
    let mut visible = HashSet::new();

    for y in 0..map.height {
        let mut max = [None; 2];
        for x in 0..map.width {
            let negx = map.width - x - 1;
            upd_vis(&map, x, y, &mut max[0], &mut visible);
            upd_vis(&map, negx, y, &mut max[1], &mut visible);
        }
    }

    for x in 0..map.width {
        let mut max = [None; 2];
        for y in 0..map.height {
            let negy = map.height - y - 1;
            upd_vis(&map, x, y, &mut max[0], &mut visible);
            upd_vis(&map, x, negy, &mut max[1], &mut visible);
        }
    }

    visible.len()
}

fn upd_vis(
    map: &Map,
    x: usize,
    y: usize,
    max: &mut Option<u8>,
    visible: &mut HashSet<(usize, usize)>,
) {
    let v = map.get(x, y);
    if max.map(|m| m < v).unwrap_or(true) {
        *max = Some(v);
        visible.insert((x, y));
    }
}

fn score(map: &Map, house: u8, mut xs: impl Iterator<Item = usize>, mut ys: impl Iterator<Item = usize>) -> usize {
    let mut score = 0;
    loop {
        match (xs.next(), ys.next()) {
            (Some(x), Some(y)) => {
                let v = map.get(x, y);
                score += 1;
                if v >= house {
                    break;
                }
            }
            _ => break,
        }
    }
    score
}

fn scenic_score(map: &Map, x: usize, y: usize) -> [usize;4] {
    let house = map.get(x, y);

    let right = score(map, house, x + 1..map.width, repeat(y));
    let left = score(map, house, (1..=x).map(|n| x - n), repeat(y));
    let up = score(map, house, repeat(x), y + 1..map.height);
    let down = score(map, house, repeat(x), (1..=y).map(|n| y - n));

    [right,left,up,down]
}

fn part2(input: &[u8]) -> usize {
    let map = parse(input);
    let mut score_max = None;
    for y in 0..map.height {
        for x in 0..map.width {
            let score_details = scenic_score(&map, x, y);
            let score = score_details.iter().product();
            if score_max.is_none() || score_max.unwrap() < score {
                score_max = Some(score);
                // dbg!(x, y, score, score_details);
            }
        }
    }
    score_max.unwrap()
}
