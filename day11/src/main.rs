use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

struct Monkey {
    items: VecDeque<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    test_mod: usize,
    true_monkey: usize,
    false_monkey: usize,
    inspected: usize,
}

fn part1(input: &str) -> usize {
    let mut monkeys = parse(input);
    for _round in 0..20 {
        for idx in 0..monkeys.len() {
            while let Some(mut item) = monkeys[idx].items.pop_front() {
                item = (monkeys[idx].op)(item) / 3;
                monkeys[idx].inspected+=1;
                let test = item % monkeys[idx].test_mod == 0;
                let dest = if test { monkeys[idx].true_monkey } else { monkeys[idx].false_monkey };
                monkeys[dest].items.push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspected);
    monkeys[monkeys.len()-1].inspected*monkeys[monkeys.len()-2].inspected
}

fn parse(input: &str) -> Vec<Monkey> {
    let mut result = vec![];
    for record in input.split("\n\n") {
        let mut items = VecDeque::new();
        let mut op = None;
        let mut test_mod = None;
        let mut true_monkey = None;
        let mut false_monkey = None;
        for line in record.lines() {
            if let Some((_, list)) = line.split_once("Starting items: ") {
                items = list
                    .split(", ")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect();
            }
            if let Some((_,opstr)) = line.split_once("Operation: new = ") {
                let eqn: Vec<_> = opstr.split_whitespace().collect();
                let op_ch = eqn[1].as_bytes()[0];
                let a = eqn[0].parse::<usize>().ok();
                let b = eqn[2].parse::<usize>().ok();
                op = Some(Box::new(move |old| {
                    let a = a.unwrap_or(old);
                    let b = b.unwrap_or(old);
                    match op_ch {
                        b'+' => a + b,
                        b'*' => a * b,
                        _ => unimplemented!(),
                    }
                }));
            }
            if let Some((_, test_str)) = line.split_once("divisible by ") {
                test_mod = Some(test_str.parse().unwrap());
            }
            if let Some((_, true_dest)) = line.split_once("true: throw to monkey ") {
                true_monkey = Some(true_dest.parse().unwrap());
            }
            if let Some((_, false_dest)) = line.split_once("false: throw to monkey ") {
                false_monkey = Some(false_dest.parse().unwrap());
            }
        }
        result.push(Monkey {
            items,
            op: op.unwrap(),
            test_mod: test_mod.unwrap(),
            true_monkey: true_monkey.unwrap(),
            false_monkey: false_monkey.unwrap(),
            inspected: 0,
        });
    }

    result
}


#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../test1")), 10605);
}


fn part2(input: &str) -> usize {
    let mut monkeys = parse(input);

    let modulus: usize = monkeys.iter().map(|m| m.test_mod).product(); 

    for _round in 0..10_000 {
        for idx in 0..monkeys.len() {
            while let Some(mut item) = monkeys[idx].items.pop_front() {
                item = (monkeys[idx].op)(item) % modulus;
                monkeys[idx].inspected+=1;
                let test = item % monkeys[idx].test_mod == 0;
                let dest = if test { monkeys[idx].true_monkey } else { monkeys[idx].false_monkey };
                monkeys[dest].items.push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspected);
    monkeys[monkeys.len()-1].inspected*monkeys[monkeys.len()-2].inspected
}