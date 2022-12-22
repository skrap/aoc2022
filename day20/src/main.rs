fn main() {
    let input = include_str!("../input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn parse(input:&str, factor: i64) -> Vec<i64> {
    input.lines().map(|l| l.parse::<i64>().unwrap() * factor).collect()
}

fn part1(input: &str) -> i64 {
    mixer(input, 1, 1)
}

fn part2(input: &str) -> i64 {
    mixer(input, 811589153, 10)
}

fn mixer(input: &str, factor: i64, rounds: usize) -> i64 {
    let orig_nums = parse(input, factor);
    let mod_n = orig_nums.len() - 1;
    // normalize to positive
    let nums: Vec<_> = orig_nums.iter().map(|&i| {
        let mut n = i % mod_n as i64;
        if n < 0 {
            n += (((-n as usize) + mod_n - 1)/mod_n*mod_n) as i64;
        }
        assert!(n >= 0 && n < mod_n as i64);
        n
    }).collect();

    let mut check = vec![0;nums.len()];
    let mut positions = Vec::from_iter(0..nums.len());
    for round in 0..rounds  {
        for (orig_idx, n) in nums.iter().enumerate() {
            // mix one number
            let old_pos = positions[orig_idx];
            let new_pos = (old_pos + *n as usize)%mod_n;
            for other_pos in positions.iter_mut() {
                if old_pos < *other_pos && *other_pos <= new_pos {
                    *other_pos -= 1;
                }
                if new_pos <= *other_pos && *other_pos < old_pos {
                    *other_pos += 1;
                }
            }
            positions[orig_idx] = new_pos;

            // for (orig_idx,now_idx) in positions.iter().enumerate() {
            //     check[*now_idx] = orig_nums[orig_idx];
            // }
            // dbg!(orig_idx, &check);
        }
    }

    let orig_zero_pos = nums.iter().position(|n| *n == 0).unwrap();
    let zero_pos = positions[orig_zero_pos];
    [1000,2000,3000].map(|offset| {
        let pos = (zero_pos + offset)%(positions.len());
        let orig_idx = positions.iter().position(|p| *p == pos).unwrap();
        orig_nums[orig_idx]
    }).iter().sum()
}

#[test]
fn test() {
    assert_eq!(part1(include_str!("../test")), 3);
    assert_eq!(part2(include_str!("../test")), 1623178306);
}