use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input, 50));
}

struct Me {
    pos: Pt,
    facing: Facing,
}

impl Me {
    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
        }
    }
    fn turn_left(&mut self) {
        self.facing = match self.facing {
            Facing::Up => Facing::Left,
            Facing::Left => Facing::Down,
            Facing::Down => Facing::Right,
            Facing::Right => Facing::Up,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

type Pt = [i64; 2];

enum Step {
    TurnRight,
    TurnLeft,
    Forward(usize),
}

fn parse(input: &str) -> (HashMap<Pt, u8>, Vec<Step>) {
    let (maptext, stepstext) = input.split_once("\n\n").unwrap();

    let mut map = HashMap::new();
    for (y, line) in maptext.lines().enumerate() {
        for (x, sym) in line.bytes().enumerate() {
            match sym {
                b'#' | b'.' => {
                    map.insert([x as i64, y as i64], sym);
                }
                _ => (),
            }
        }
    }

    let stepstext = stepstext.trim_end(); // take off linefeed
    let mut steps = vec![];
    let mut start = 0;
    for (idx, ch) in stepstext.char_indices() {
        if ch == 'R' || ch == 'L' {
            if idx > start {
                steps.push(Step::Forward(stepstext[start..idx].parse().unwrap()));
            }
            steps.push(match ch {
                'R' => Step::TurnRight,
                'L' => Step::TurnLeft,
                _ => unimplemented!(),
            });
            start = idx + 1;
        }
    }
    if start != stepstext.len() - 1 {
        steps.push(Step::Forward(
            stepstext[start..stepstext.len()].parse().unwrap(),
        ));
    }

    (map, steps)
}

fn part1(input: &str) -> usize {
    let (map, steps) = parse(input);
    dance(map, steps, None)
}

fn part2(input: &str, cube_size: usize) -> usize {
    let (map, steps) = parse(input);
    dance(map, steps, Some(cube_size))
}

fn dance(map: HashMap<Pt, u8>, steps: Vec<Step>, cube_size: Option<usize>) -> usize {
    let start_x = (0..).find(|x| map.get(&[*x, 0]) == Some(&b'.')).unwrap();

    let mut bounds = [0, 0];
    map.keys().for_each(|[x, y]| {
        bounds[0] = bounds[0].max(*x);
        bounds[1] = bounds[1].max(*y);
    });

    let mut me = Me {
        pos: [start_x, 0],
        facing: Facing::Right,
    };

    for step in &steps {
        match step {
            Step::TurnRight => me.turn_right(),
            Step::TurnLeft => me.turn_left(),
            Step::Forward(n) => {
                for _ in 0..*n {
                    let mut nextpt = me.pos;
                    match me.facing {
                        Facing::Up => nextpt[1] -= 1,
                        Facing::Down => nextpt[1] += 1,
                        Facing::Left => nextpt[0] -= 1,
                        Facing::Right => nextpt[0] += 1,
                    };
                    let mut facing = me.facing;
                    if let None = map.get(&nextpt) {
                        if let Some(cube_size) = cube_size {
                            (nextpt, facing) =
                                wrap_part2(&me, nextpt, cube_size as i64);
                        } else {
                            nextpt = wrap_part1(&me, nextpt, bounds, &map);
                        }
                    }
                    match map.get(&nextpt) {
                        Some(b'.') => {
                            me.pos = nextpt;
                            me.facing = facing;
                        }
                        Some(b'#') => break,
                        _ => unimplemented!(),
                    }
                }
            }
        }
    }

    (1000 * (me.pos[1] + 1)
        + 4 * (me.pos[0] + 1)
        + match me.facing {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        })
    .try_into()
    .unwrap()
}

fn wrap_part1(me: &Me, mut nextpt: Pt, bounds: [i64; 2], map: &HashMap<Pt, u8>) -> Pt {
    // wrap
    match me.facing {
        Facing::Up => {
            nextpt[1] = bounds[1];
            while let None = map.get(&nextpt) {
                nextpt[1] -= 1;
                assert!(nextpt[1] >= 0);
            }
        }
        Facing::Down => {
            nextpt[1] = 0;
            while let None = map.get(&nextpt) {
                nextpt[1] += 1;
                assert!(nextpt[1] <= bounds[1]);
            }
        }
        Facing::Left => {
            nextpt[0] = bounds[0];
            while let None = map.get(&nextpt) {
                nextpt[0] -= 1;
                assert!(nextpt[0] >= 0);
            }
        }
        Facing::Right => {
            nextpt[0] = 0;
            while let None = map.get(&nextpt) {
                nextpt[0] += 1;
                assert!(nextpt[0] <= bounds[0]);
            }
        }
    }
    nextpt
}


#[test]
fn test_wrap_part2() {
    let (map, _) = parse(include_str!("../input"));
    
    let cube_size = 50;

    for tile_y in 0..3 {
        for tile_x in 0..4 {
            let pos = [10 + cube_size*tile_x ,10 + cube_size*tile_y];
            if let None = map.get(&pos) {
                continue;
            }
            for facing in [Facing::Up, Facing::Down, Facing::Left, Facing::Right] {
                let mut me = Me { pos, facing };
                for _ in 0..(cube_size*4) {
                    match me.facing {
                        Facing::Up => me.pos[1] -= 1,
                        Facing::Down => me.pos[1] += 1,
                        Facing::Left => me.pos[0] -= 1,
                        Facing::Right => me.pos[0] += 1,
                    };
                    if let None = map.get(&me.pos) {
                        let n = wrap_part2(&me, me.pos, cube_size);
                        me.pos = n.0;
                        me.facing = n.1;
                    }
                }
                assert_eq!(me.pos, pos);
                assert_eq!(me.facing, facing);
            } 
        }
    }
}

fn wrap_part2(
    me: &Me,
    mut nextpt: Pt,
    cube_size: i64,
) -> (Pt, Facing) {
    // wrap.
    // these rules are only for my input.  for a general solution, we'd need to normalize the cube
    // via translations or mirroring of each of the faces.
    let side = [nextpt[0] / cube_size, nextpt[1] / cube_size];
    let tile_pos = [nextpt[0] % cube_size, nextpt[1] % cube_size];
    let tile_pos_inv = [cube_size - tile_pos[0] - 1, cube_size - tile_pos[1] - 1];
    let facing;
    match (&me.facing, side[0], side[1]) {
        (Facing::Left, 0, 0) => {
            nextpt = [0, cube_size*2+tile_pos_inv[1]];
            facing = Facing::Right;
        }
        (Facing::Up, 0, 1) => {
            nextpt = [cube_size, tile_pos[0] + cube_size];
            facing = Facing::Right;
        }
        (Facing::Left, 0, 2) => {
            nextpt = [cube_size,tile_pos_inv[1]];
            facing = Facing::Right;
        }
        (Facing::Up, 2, 0) => {
            facing = Facing::Up;
            nextpt = [tile_pos[0], cube_size*4-1];
        }
        (Facing::Down, 0, 4) => {
            facing = Facing::Down;
            nextpt = [tile_pos[0] + cube_size*2, 0];
        }
        (Facing::Up, 1 ,0) => {
            facing = Facing::Right;
            nextpt = [0, tile_pos[0] + cube_size*3]
        }
        (Facing::Left, 0, 3) => {
            facing = Facing::Down;
            nextpt = [tile_pos[1] + cube_size,0];
        }
        (Facing::Left, 0, 1) => {
            facing = Facing::Down;
            nextpt = [tile_pos[1], cube_size*2];
        }
        (Facing::Right, 2, 1) => {
            facing = Facing::Up;
            nextpt = [tile_pos[1]+cube_size*2,cube_size-1];
        }
        (Facing::Down, 2,1) => {
            facing = Facing::Left;
            nextpt = [cube_size*2-1, cube_size+tile_pos[0]];
        }
        (Facing::Right, 3, 0) => {
            facing = Facing::Left;
            nextpt = [cube_size*2-1, cube_size*2+tile_pos_inv[1]];
        }
        (Facing::Right, 2, 2) => {
            facing = Facing::Left;
            nextpt = [cube_size*3-1,tile_pos_inv[1]];
        }
        (Facing::Down, 1, 3) => {
            facing = Facing::Left;
            nextpt = [cube_size-1, cube_size*3+tile_pos[0]];
        }
        (Facing::Right,1,3) => {
            facing = Facing::Up;
            nextpt = [cube_size+tile_pos[1],cube_size*3-1];
        }
        _ => unimplemented!(),
    }
    (nextpt, facing)
}

#[test]
fn test() {
    assert_eq!(part1(include_str!("../test")), 6032);
}
