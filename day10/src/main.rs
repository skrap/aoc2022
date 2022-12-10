struct CPU {
    prog: Vec<Instr>,
    cyc: i32,
    x: i32,
    cyc_at_pc: usize,
    pc: usize,
}

impl CPU {
    fn tick(&mut self) -> Option<(i32,i32)> { 
        let instr = self.prog.get(self.pc)?;
        self.cyc += 1;
        self.cyc_at_pc += 1;
        let ss = (self.cyc as i32, self.x);

        let done = match instr {
            Instr::NoOp => true,
            Instr::AddX(v) if self.cyc_at_pc == 2 => {
                self.x += v;
                true
            },
            _ => false,
        };
        if done {
           self.pc += 1;
           self.cyc_at_pc = 0; 
        }
        Some(ss)
    }
}

enum Instr {
    NoOp,
    AddX(i32),
}

impl Instr {
    fn parse(line: &str) -> Self {
        match line.split_once(" ") {
            None if line == "noop" => Self::NoOp,
            Some(("addx", val)) => Self::AddX(val.parse().unwrap()),
            _ => unimplemented!()
        }
    }
}

fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> i32 {
    let prog: Vec<_> = input.lines().map(Instr::parse).collect();
    let mut cpu = CPU {
        prog,
        cyc: 0,
        x: 1,
        cyc_at_pc: 0,
        pc: 0,
    };

    let mut sig_str = 0;
    while let Some(ss) = cpu.tick() {
        if cpu.cyc%40 == 20 {
            // dbg!(cpu.cyc, ss);
            sig_str += ss.0*ss.1;
        }
    }

    sig_str
}

#[test]
fn test_part1() {
    let input = include_str!("../test1");
    assert_eq!(part1(input), 13140);
}

fn part2(input: &str) {
    let prog: Vec<_> = input.lines().map(Instr::parse).collect();
    let mut cpu = CPU {
        prog,
        cyc: 0,
        x: 1,
        cyc_at_pc: 0,
        pc: 0,
    };

    let mut output = vec![];
    while let Some((cyc, x)) = cpu.tick() {
        let ch = if ((cyc%40) - x - 1).abs() <= 1 {
            b'#'
        } else {
            b' '
        };
        output.push(ch);
    }
    for line in output.chunks(40) {
        println!("{}",std::str::from_utf8(line).unwrap());
    }
}