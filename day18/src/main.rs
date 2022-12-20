fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn parse(input: &str) -> Vec<u32> {
    input.lines().map(
        |line| {
            let mut it = line.split(",").map(|n| n.parse::<u8>().unwrap());
            u32::from_be_bytes([0,it.next().unwrap()+1, it.next().unwrap()+1, it.next().unwrap()+1])
        }
    ).collect()
}

fn part1(input: &str) -> usize {
    let mut pts = parse(input);
    let mut area = 0;

    let sort_keys = [
        |n: &u32| { *n },
        |n: &u32| {
            let [_,x,y,z] = n.to_be_bytes();
            u32::from_be_bytes([0,x,z,y])
        },
        |n: &u32| {
            let [_,x,y,z] = n.to_be_bytes();
            u32::from_be_bytes([0,y,z,x])
        }
    ];

    for sort_idx in [2,1,0] {
        area += 2; // for start and end
        pts.sort_by_key(sort_keys[sort_idx]);
        for pair in pts.windows(2) {
            if pair[1].wrapping_sub(pair[0]) != 1 << 8*sort_idx {
                area += 2; // for end and start
            }
        }
    }

    area
}

#[test]
fn test_part1() {
    let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    assert_eq!(part1(input),64);
}
#[test]
fn test_part2() {
    let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    assert_eq!(part2(input),58);
}

fn part2(input: &str) -> usize {
    // just do a flood fill i guess
    let pts = parse(input);
    let mut map = vec![false;256*256*256];
    let mut checked = map.clone();
    for pt in &pts {
        map[*pt as usize] = true;
    }

    let extent = (0..3).map(|dim| {
        let max = pts.iter().map(|u| u.to_be_bytes()[1+dim]).max().unwrap();
        max
    }).max().unwrap();

    fn check(pt: [u8;3], map: &[bool], checked: &mut [bool], extent: u8) -> usize {
        fn idx(pt: [u8;3]) -> usize {
            u32::from_be_bytes([0,pt[0],pt[1],pt[2]]) as usize
        }
        if checked[idx(pt)] {
            // don't double-count
            return 0;
        }
        checked[idx(pt)] = true;

        let mut total = 0;
        for dim in 0..3 {
            if let Some(n) = pt[dim].checked_sub(1) {
                let mut pt = pt;
                pt[dim] = n;
                if map[idx(pt)] {
                    total += 1;
                } else {
                    total += check(pt, map, checked, extent);
                }
            }
            if pt[dim] <= extent {
                let mut pt = pt;
                pt[dim] += 1;
                if map[idx(pt)] {
                    total += 1;
                } else {
                    total += check(pt, map, checked, extent);
                }
            }
        }
        
        total
    }

    check([0,0,0], &map, &mut checked, extent)
}