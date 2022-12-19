use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2_memo(input));
}

type Pos = [u8; 2];

struct Valve {
    rate: usize,
    tunnels: Vec<Pos>,
}

fn parse(input: &str) -> HashMap<Pos, Valve> {
    let mut res = HashMap::new();
    for line in input.lines() {
        let (rate, tunnels) = line.split_once(";").unwrap();
        let name: Pos = rate.split(" ").skip(1).next().unwrap().as_bytes().try_into().unwrap();
        let rate = rate.split_once("=").unwrap().1.parse().unwrap();
        let tunnels = if tunnels.contains("valves") {
            tunnels
                .split_once("valves ")
                .unwrap()
                .1
                .split(", ")
                .map(|s| {
                    assert_eq!(s.len(), 2);
                    s.as_bytes().try_into().unwrap()
                })
                .collect()
        } else {
            let name = tunnels.split_once("valve ").unwrap().1;
            vec![name.as_bytes().try_into().unwrap()]
        };
        res.insert(name, Valve { rate, tunnels });
    }
    res
}


fn part1_memo(input: &str) -> usize {
    let map = parse(input);
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Me {
        time_left: u8,
        pos: Pos,
        to_open: Vec<Pos>,
    }
    let mut to_open: Vec<_> = map
        .iter()
        .filter_map(
            |(name, node)| {
                if node.rate > 0 {
                    Some(*name)
                } else {
                    None
                }
            },
        )
        .collect();
    to_open.sort();
    let start = Me {
        time_left: 30,
        pos: *b"AA",
        to_open,
    };
    let mut memos = Default::default();
    fn max_press<'map>(
        me: Me,
        map: &'map HashMap<Pos, Valve>,
        memos: &mut HashMap<Me, usize>,
        dbg: &mut u8,
    ) -> usize {
        if let Some(max) = memos.get(&me) {
            return *max;
        }
        if me.time_left == 0 {
            return 0;
        }
        let mut max = 0;
        let here = map.get(&me.pos).unwrap();

        // 1: open a valve
        if me.to_open.contains(&me.pos) {
            let to_open = me
                .to_open
                .iter()
                .filter(|r| **r != me.pos)
                .cloned()
                .collect();
            let time_left = me.time_left - 1;
            let flow = here.rate * time_left as usize;
            max = max.max(flow + max_press(
                Me {
                    to_open,
                    time_left,
                    pos: me.pos,
                },
                map,
                memos,
                dbg,
            ));
        }
        // 2: travel a tunnel
        if me.to_open.len() > 0 {
            for &pos in &here.tunnels {
                let time_left = me.time_left - 1;
                max = max.max(max_press(
                    Me {
                        time_left,
                        pos,
                        ..me.clone()
                    },
                    map,
                    memos,
                    dbg,
                ));
            }
        }
        // 3: just sit here forever
        if *dbg < me.time_left {
            *dbg = dbg!(me.time_left);
        }
        memos.insert(me, max);
        max
    }

    let mut dbg = 0;
    let answer = max_press(start, &map, &mut memos, &mut dbg);
    answer
}

fn part1(input: &str) -> usize {
    part1_memo(input)
}

#[test]
fn test() {
    assert_eq!(part1(include_str!("../test1")), 1651);
}

#[test]
fn test_part2() {
    assert_eq!(part2_memo(include_str!("../test1")), 1707);
}

fn part2_memo(input: &str) -> usize {
    let map = parse(input);
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Team {
        time_left: u8,
        pos: [Pos;2],
        to_open: Vec<Pos>,
    }
    let to_open: Vec<_> = map
        .iter()
        .filter_map(
            |(name, node)| {
                if node.rate > 0 {
                    Some(*name)
                } else {
                    None
                }
            },
        )
        .collect();
    let start = Team {
        time_left: 26,
        pos: [*b"AA";2],
        to_open,
    };
    let mut memos = Default::default();
    fn max_press<'map>(
        team: Team,
        map: &'map HashMap<Pos, Valve>,
        memos: &mut HashMap<Team, usize>,
        dbg: &mut u8,
    ) -> usize {
        if let Some(max) = memos.get(&team) {
            return *max;
        }
        if team.time_left == 0 {
            return 0;
        }

        #[derive(Clone, Copy)]
        enum Act {
            Valve(usize),
            Move(usize,Pos),
            Sit
        }
        use Act::*;

        let mut acts = [vec![], vec![]];
        let here = [&map[&team.pos[0]],&map[&team.pos[1]]];
        for i in 0..2 {
            if team.to_open.contains(&team.pos[i]) {
                acts[i].push(Valve(i));
            }
            if team.to_open.len() > 0 {
                acts[i].extend(here[i].tunnels.iter().map(|p| Move(i,*p)));
            }
            if acts[i].is_empty() {
                acts[i].push(Sit);
            }
        }

        let actions = acts[0].iter()
            .flat_map(|a0| acts[1].iter().map(|a1| [*a0,*a1]));

        let max = actions.map(|acts| {
            let mut team = team.clone();
            let mut press = 0;
            team.time_left -= 1;

            if matches!(acts,[Sit, Sit]) {
                return 0;
            }

            for act in acts.into_iter() {
                match act {
                    Valve(idx) => {
                        if let Some(valve_idx) = team.to_open.iter().position(|p| *p == team.pos[idx]) {
                            press += here[idx].rate * team.time_left as usize;
                            team.to_open.remove(valve_idx);
                        }
                    },
                    Move(idx, pos) => {
                        team.pos[idx] = pos;
                    },
                    Sit => (),
                }
            }
            press + max_press(team, map, memos, dbg)
        }).max().unwrap();

        if *dbg < team.time_left {
            *dbg = dbg!(team.time_left);
        }
        memos.insert(team, max);
        max
    }

    let mut dbg = 0;
    let answer = max_press(start, &map, &mut memos, &mut dbg);
    answer
}
