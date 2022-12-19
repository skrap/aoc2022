use std::collections::HashMap;

fn main() {
    let input = include_str!("../input").trim().as_bytes();
    dbg!(part1(input));
    dbg!(part2(input));
}

const ROCKS: [u32; 5] = [
    // high bit in each byte is the wraparound detector
    0b000_11110,
    u32::from_be_bytes([0, 0b000_01000, 0b000_11100, 0b000_01000]),
    u32::from_be_bytes([0, 0b000_00100, 0b000_00100, 0b000_11100]),
    u32::from_be_bytes([0b000_10000, 0b000_10000, 0b000_10000, 0b000_10000]),
    u32::from_be_bytes([0, 0, 0b000_11000, 0b000_11000]),
];

fn part1(input: &[u8]) -> usize {
    let mut map = vec![]; // start with a floor

    let mut gusts = input.iter().copied().enumerate().cycle();
    let rocks = ROCKS.into_iter().cycle();

    for rock in rocks.take(2022) {
        step(&mut map, &mut gusts, rock);
    }

    let zeroes = map.iter().rev().take_while(|b| **b == 0).count();
    map.len() - zeroes
}

fn step(map: &mut Vec<u8>, gusts: &mut impl Iterator<Item = (usize,u8)>, mut rock: u32) {
    const WALL_DETECTOR: u32 = u32::from_le_bytes([0b1000_0000; 4]);

    // resize our map to hold new rock
    let mut zeroes = map.iter().rev().take_while(|b| **b == 0).count();
    while zeroes < 7 {
        map.push(0);
        zeroes += 1;
    }
    let mut height = map.len() - 4;
    loop {
        // push by gust
        let pushed_rock = match gusts.next().unwrap() {
            (_,b'<') => rock.rotate_left(1),
            (_,b'>') => rock.rotate_right(1),
            _ => unimplemented!(),
        };
        let other_rocks = u32::from_le_bytes(map[height..height + 4].try_into().unwrap());
        if (pushed_rock & (WALL_DETECTOR | other_rocks)) == 0 {
            // didn't hit a wall or another rock.
            rock = pushed_rock;
        }

        // try to fall 1 space
        if height == 0 {
            map[height..height + 4].copy_from_slice((rock | other_rocks).to_le_bytes().as_slice());
            break;
        }
        let lower_rocks = u32::from_le_bytes(map[height - 1..height + 3].try_into().unwrap());
        if (rock & lower_rocks) != 0 {
            map[height..height + 4].copy_from_slice((rock | other_rocks).to_le_bytes().as_slice());
            break;
        }
        height -= 1;
    }
    //print_map(&map);
}

#[allow(unused)]
fn print_map(map: &[u8]) {
    println!();
    for mut b in map.iter().rev().copied() {
        let mut s = String::new();
        for _ in 0..7 {
            s.push(if (b & 0b1000000) != 0 { '#' } else { ' ' });
            b <<= 1;
        }
        println!("|{}|", s);
    }
}

fn part2(input: &[u8]) -> usize {
    let mut map = vec![]; // start with a floor

    let mut gusts = input.iter().copied().enumerate().cycle().peekable();
    let rocks = ROCKS.into_iter().cycle();

    let mut flat_rows = HashMap::new();
    let mut offset = None;
    let rock_limit = 1000000000000;

    for (idx,rock) in rocks.enumerate() {
        if let Some((rock_offset,height_offset)) = &offset {
            if idx + rock_offset == rock_limit {
                let zeroes = map.iter().rev().take_while(|b| **b == 0).count();
                return map.len() - zeroes + height_offset;
            }
        }
        step(&mut map, &mut gusts, rock);
        // find the top and see if we completed a row
        if offset.is_none() {
            if let Some(row) = map.iter().rev().find(|r| **r != 0) {
                if *row == 0b01111111u8 {
                    let key = (gusts.peek().unwrap().0-1,rock);
                    let (old_idx, old_height) = flat_rows.entry(key).or_insert((idx,map.len()));
                    if *old_idx != idx {
                        println!("repeat at {:?}:", key);
                        let delta_rocks = dbg!(idx - *old_idx);
                        let delta_height = dbg!(map.len() - *old_height);
                        let loops = (rock_limit-idx)/delta_rocks;
                        offset = Some((loops*delta_rocks, loops*delta_height));
                    }
                }
            }
        }
    }
    unreachable!()
}

#[test]
fn test() {
    assert_eq!(
        part1(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".as_bytes()),
        3068
    );
    // assert_eq!(
    //     part2(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".as_bytes()),
    //     1514285714288
    // );
}
