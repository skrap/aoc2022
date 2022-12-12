use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    #[cfg(not(feature = "animate"))]
    dbg!(fling_rope(input, 2));
    dbg!(fling_rope(input, 10));
}

#[test]
fn test_part1() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(fling_rope(input, 2), 13);
    assert_eq!(fling_rope(input, 10), 1);
}

fn fling_rope(input: &str, rope_len: usize) -> usize {
    let mut rope = vec![[0i32,0]; rope_len];

    let mut visited = HashSet::new();
    visited.insert(*rope.last().unwrap());

    for line in input.lines() {
        let (dir, len) = line.split_once(" ").unwrap();
        let len : i32 = len.parse().unwrap();

        for _ in 0..len {
            match dir {
                "R" => rope[0][0] += 1,
                "L" => rope[0][0] -= 1,
                "U" => rope[0][1] += 1,  // math-style
                "D" => rope[0][1] -= 1,
                _ => unimplemented!(),
            }

            for i0 in 0..rope.len()-1 {
                let head = rope[i0];
                let tail = &mut rope[i0+1];
                let [dx, dy] = [head[0]-tail[0], head[1]-tail[1]];
                if dx.abs() > 1 || dy.abs() > 1 {
                    tail[0] += dx.signum();
                    tail[1] += dy.signum();
                }
            }

            visited.insert(*rope.last().unwrap());

            #[cfg(feature = "animate")]
            draw_frame(&rope);
        }
    }

    visited.len()
}

#[allow(unused)]
fn draw_frame(rope: &[[i32;2]]) {
    let head = rope[0];
    let tail = *rope.last().unwrap();
    let mut rx = [head[0];2];
    let mut ry = [head[1];2];
    for knot in &rope[1..] {
        rx[0] = rx[0].min(knot[0]);
        rx[1] = rx[1].max(knot[0]);
        ry[0] = ry[0].min(knot[1]);
        ry[1] = ry[1].max(knot[1]);
    }

    let mut screen = format!("{}", ansi_escapes::ClearScreen);
    for y in -20..=ry[1] {
        for x in -50..=rx[1] {
            let pt = [x,y];
            let ch = if pt == head {
                'H'
            } else if pt == tail {
                'T'
            } else if rope.contains(&pt) {
                '#'
            } else {
                ' '
            };
            screen.push(ch);
        }
        screen.push('\n')
    }
    print!("{}", screen);
    std::thread::sleep(std::time::Duration::from_millis(10));
}
