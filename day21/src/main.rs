use std::{collections::{HashMap, VecDeque}, cell};

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

/// whee it's baby excel maybe
#[derive(Copy, Clone)]
enum Cell {
    Constant(i64),
    Formula(usize,char,usize),
    Human,
}

fn parse(input: &str, part2_mods: bool) -> (Vec<Cell>,HashMap<String,usize>) {
    let mut namemap = HashMap::new();
    let mut names = vec![];
    for line in input.lines() {
        let (name,_) = line.split_once(':').unwrap();
        names.push(name.to_string());
        namemap.insert(name.to_string(), names.len()-1);
    }

    let mut cells = vec![];
    for line in input.lines() {
        // sample lines:
        // pbcg: frwf - vzrg
        // gfhs: 4
        let tokens: Vec<_> = line.split_whitespace().collect();
        let formula = if part2_mods && tokens[0] == "humn:" {
            Cell::Human
        } else {
            match tokens.len() {
                2 => Cell::Constant(tokens[1].parse().unwrap()),
                4 =>{
                    Cell::Formula(
                        namemap[tokens[1]], tokens[2].chars().next().unwrap(), namemap[tokens[3]])
                },
                _ => unimplemented!(),
            }
        };
        cells.push(formula);
    }
    (cells,namemap)
}

fn part1(input: &str) -> i64 {
    let (mut cells, names) = parse(input, false);
    
    let mut work = VecDeque::new();
    work.push_front(names["root"]);

    while let Some(cell_idx) = work.pop_front() {
        match cells[cell_idx] {
            Cell::Human => (),
            Cell::Constant(_) => (),
            Cell::Formula(left, op, right) => {
                match (&cells[left], &cells[right]) {
                    (Cell::Constant(left), Cell::Constant(right)) => {
                        let val = match op {
                            '*' => left * right,
                            '+' => left + right,
                            '-' => left - right,
                            '/' => left / right,
                            _ => unimplemented!(),
                        };
                        cells[cell_idx] = Cell::Constant(val);
                    },
                    _ => {
                        work.push_front(cell_idx);
                        work.push_front(right);
                        work.push_front(left);
                    }
                }
            }
        }
    }

    match cells[names["root"]] {
        Cell::Constant(val) => val,
        Cell::Formula(_, _, _) => todo!(),
        Cell::Human => todo!(),
    }
}


fn part2(input: &str) -> i64 {
    use Cell::*;

    let (mut cells, names) = parse(input, true);
    
    let mut work = VecDeque::new();
    let (left,right) = match cells[names["root"]] {
        Constant(_) => todo!(),
        Formula(left, _, right) => {
            work.extend([left,right]);
            (left, right)
        }
        Human => todo!(),
    };

    // this is an inelegant hack but i need to sleep
    let mut seen = vec![false;cells.len()];

    // simplify
    while let Some(cell_idx) = work.pop_front() {
        match cells[cell_idx] {
            Human => (),
            Constant(_) => (),
            Formula(left, op, right) => {
                match (&cells[left], &cells[right]) {
                    (Constant(left), Constant(right)) => {
                        let val = match op {
                            '*' => left * right,
                            '+' => left + right,
                            '-' => left - right,
                            '/' => left / right,
                            _ => unimplemented!(),
                        };
                        cells[cell_idx] = Constant(val);
                    },
                    (Human, _) => {
                        work.push_front(right);
                    }
                    (_, Human) => {
                        work.push_front(left);
                    }
                    _ => {
                        if seen[cell_idx] {
                            continue;
                        }
                        seen[cell_idx] = true; // only traverse this node once
                        work.push_front(cell_idx);
                        work.push_front(right);
                        work.push_front(left);
                    }
                }
            }
        }
    }

    let (left_cell, right_cell) = (cells[left], cells[right]);
    let (mut known, mut unknown) = match (left_cell, right_cell) {
        (Constant(known), unknown @ Formula{..}) => (known, unknown),
        (unknown @ Formula{..}, Constant(known)) => (known, unknown),
        _ => unimplemented!(),
    };

    while let Formula(left, op, right) = unknown {
        let cells = (&cells[left], op, &cells[right]);
        unknown = match cells {
            (Constant(a), '+', unknown) | (unknown, '+', Constant(a))=> {
                known -= a;
                *unknown
            },
            (Constant(a), '*', unknown) | (unknown, '*', Constant(a))=> {
                known /= a;
                *unknown
            },
            (Constant(a), '-', unknown) => {
                known = -(known - a);
                *unknown
            },
            (unknown, '-', Constant(a)) => {
                known += a;
                *unknown
            },
            (Constant(a), '/', unknown) => {
                known = a/known;
                *unknown
            },
            (unknown, '/', Constant(a)) => {
                known *= a;
                *unknown
            },
            _=> unimplemented!()
        }
    }

    known
}


