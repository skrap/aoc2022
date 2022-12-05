fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

#[test]
fn test_part1() {
    let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(part1(input), "CMZ".to_string());
}

fn part1(input: &str) -> String {
    let (mut stacks, moves) = parse(input);
    for mov in moves {
        for _n in 0..mov.count {
            let cr = stacks[mov.from-1].pop().unwrap();
            stacks[mov.to-1].push(cr);
        }
    }

    let mut result = String::new();
    for s in stacks {
        result.push(*s.last().unwrap());
    }
    result
}


fn part2(input: &str) -> String {
    let (mut stacks, moves) = parse(input);
    for mov in moves {
        for _n in 0..mov.count {
            let cr = stacks[mov.from-1].pop().unwrap();
            stacks[mov.to-1].push(cr);
        }
        // part 2 adaptation: last N added to `to` stack must be reversed
        let s = &mut stacks[mov.to-1];
        let range = s.len()-mov.count..s.len();
        s[range].reverse();
    }

    let mut result = String::new();
    for s in stacks {
        result.push(*s.last().unwrap());
    }
    result
}

type Stack = Vec<char>;
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse(input: &str) -> (Vec<Stack>, Vec<Move>) {
    let mut stacks = vec![];
    let mut moves = vec![];

    let (stacks_str, moves_str) = input.split_once("\n\n").unwrap();

    for line in stacks_str.trim_end().lines() {
        for (idx, s) in line.as_bytes().chunks(4).enumerate() {
            let crate_name = s[1];
            if crate_name.is_ascii_alphabetic() {
                if idx >= stacks.len() {
                    stacks.resize(idx+1, vec![]);
                }
                stacks[idx].push(crate_name as char);
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }

    for line in moves_str.trim().lines() {
        let words: Vec<_> = line.split_whitespace().collect();
        moves.push(Move {
            count: words[1].parse().unwrap(),
            from: words[3].parse().unwrap(),
            to: words[5].parse().unwrap(),
        });
    }

    (stacks, moves)
}