use std::collections::{BinaryHeap, VecDeque};

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

struct Blueprint {
    id: usize,
    costs: [[u8; 4]; 4],
}

fn parse(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<_> = line
                .split([' ', ':'])
                .filter_map(|s| s.parse::<u8>().ok())
                .collect();
            let id = nums[0].into();
            let costs = [
                [nums[1], 0, 0, 0],       // ore bot
                [nums[2], 0, 0, 0],       // clay bot
                [nums[3], nums[4], 0, 0], // obsidian bot
                [nums[5], 0, nums[6], 0], // geode bot
            ];

            Blueprint { id, costs }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let prints = parse(input);
    prints.iter().map(|print| {
        max_geodes(
            State {
                time: 24,
                resources: [0, 0, 0, 0],
                bots: [1, 0, 0, 0],
            },
            print,
        ) * print.id
    }).sum()
}

fn part2(input: &str) -> usize {
    let prints = parse(input);
    prints.iter().take(3).map(|print| {
        dbg!(max_geodes(
            State {
                time: 32,
                resources: [0, 0, 0, 0],
                bots: [1, 0, 0, 0],
            },
            print,
        ))
    }).product()
}

const ORE: usize = 0;
const CLY: usize = 1;
const OBS: usize = 2;
const GEO: usize = 3;

#[derive(Debug, PartialEq, Eq)]
struct State {
    time: u8,
    resources: [u8; 4],
    bots: [u8; 4],
}

fn build(state: &State, bot: usize, print: &Blueprint) -> Option<State> {
    let costs = print.costs[bot];
    let State {
        time,
        resources,
        bots,
    } = *state;
    if !(0..4).all(|r| resources[r] >= costs[r] || bots[r] > 0) {
        // can't build this no matter how long we wait.
        return None;
    }
    // should we build a bot of this type?  not if we already have enough to cover the needs of each input.
    if bot != GEO && (0..4).all(|rsc| print.costs[rsc][bot] <= bots[bot]) {
        return None;
    }

    // how many rounds must we remain idle before having the needed resources?
    let idle_rounds = (0..4)
        .map(|r| {
            let have = resources[r];
            let need = costs[r];
            if need > have {
                (need - have + bots[r] - 1) / bots[r]
            } else {
                0
            }
        })
        .max()
        .unwrap();
    if idle_rounds >= time.into() {
        None
    } else {
        let State {
            mut time,
            mut resources,
            mut bots,
        } = *state;

        let rounds = idle_rounds + 1;
        (0..4).for_each(|r| resources[r] = resources[r] + bots[r] * rounds - costs[r]);
        bots[bot] += 1;
        time -= u8::try_from(rounds).unwrap();
        Some(State {
            time,
            resources,
            bots,
        })
    }
}

fn max_geodes(state: State, print: &Blueprint) -> usize {
    if state.time == 0 {
        return state.resources[GEO].into();
    }

    let mut max = (state.resources[GEO] + state.bots[GEO] * state.time) as usize; // do-nothing case
                                                                                  // try to build each kind of bot
    for bot in [GEO, OBS, CLY, ORE] {
        if let Some(newstate) = build(&state, bot, print) {
            assert!(newstate.time < state.time);
            max = max.max(max_geodes(newstate, print));
        }
    }
    max
}

#[test]
fn test() {
    let input = include_str!("../test");
    assert_eq!(part1(input), 33);
    assert_eq!(part2(input), 56*62);
}
