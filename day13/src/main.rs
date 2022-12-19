use std::cmp::Ordering;

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

#[derive(PartialEq)]
enum Node {
    List(Vec<Node>),
    Int(i32),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match Node::cmp(self, other) {
            Some(true) => Some(Ordering::Less),
            Some(false) => Some(Ordering::Greater),
            None => None,
        }
    }
}

impl Node {
    fn cmp(left: &Self, right: &Self) -> Option<bool> {
        use Node::*;
        match (left, right) {
            // If both values are integers, the lower integer should come first. If the left integer is lower than the right integer, the inputs are in the right order. If the left integer is higher than the right integer, the inputs are not in the right order. Otherwise, the inputs are the same integer; continue checking the next part of the input.
            (Int(l), Int(r)) => match l.cmp(r) {
                std::cmp::Ordering::Less => Some(true),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(false),
            },

            // If both values are lists, compare the first value of each list, then the second value, and so on. If the left list runs out of items first, the inputs are in the right order. If the right list runs out of items first, the inputs are not in the right order. If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.
            (List(l), List(r)) => {
                for pair in l.iter().zip(r.iter()) {
                    if let Some(res) = Self::cmp(pair.0, pair.1) {
                        return Some(res);
                    }
                }
                match l.len().cmp(&r.len()) {
                    std::cmp::Ordering::Less => Some(true),
                    std::cmp::Ordering::Equal => None,
                    std::cmp::Ordering::Greater => Some(false),
                }
            }

            // If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison. For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2); the result is then found by instead comparing [0,0,0] and [2].
            (l @ List(..), Int(r)) => Self::cmp(l, &Node::List(vec![Node::Int(*r)])),
            (Int(l), r @ List(..)) => Self::cmp(&Node::List(vec![Node::Int(*l)]), r),
        }
    }
}

fn parse(line: &str) -> (Node, &str) {
    if let Some(mut rest) = line.strip_prefix("[") {
        let mut nodes = vec![];
        while !rest.starts_with("]") {
            let res = parse(rest);
            nodes.push(res.0);
            rest = res.1;
            if let Some(r) = rest.strip_prefix(",") {
                rest = r;
            } else {
                break;
            }
        }
        rest = rest.strip_prefix("]").unwrap();
        (Node::List(nodes), rest)
    } else {
        let idx = line.find(|ch: char| !ch.is_numeric()).unwrap();
        let (num, rest) = line.split_at(idx);
        (Node::Int(num.parse().unwrap()), rest)
    }
}

fn part1(input: &str) -> usize {
    let mut res = 0;
    for (idx, group) in input.split("\n\n").enumerate() {
        let (l, r) = group.split_once("\n").unwrap();
        if Node::cmp(&parse(l).0, &parse(r).0).unwrap() {
            res += idx + 1;
        }
    }
    res
}

fn part2(input: &str) -> usize {
    let marker = |id| Node::List(vec![Node::List(vec![Node::Int(id)])]);
    let mut msgs = vec![];
    for line in input.lines() {
        if !line.is_empty() {
            msgs.push(parse(line).0);
        }
    }
    msgs.push(marker(2));
    msgs.push(marker(6));
    msgs.sort_by(|l, r| l.partial_cmp(r).unwrap());
    (msgs.iter().position(|n| n == &marker(2)).unwrap() + 1)
        * (msgs.iter().position(|n| n == &marker(6)).unwrap() + 1)
}
