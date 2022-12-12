fn main() {
    let input = include_str!("../input").trim();
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        let nums: Vec<_> = line.split(&[',','-']).map(|s| s.parse::<u32>().unwrap()).collect();

        if nums[0] <= nums[2] && nums[1] >= nums[3] {
            count += 1;
        } else if nums[0] >= nums[2] && nums[1] <= nums[3] {
            count += 1;
        }
    }
    count
}


fn part2(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        let nums: Vec<_> = line.split(&[',','-']).map(|s| s.parse::<u32>().unwrap()).collect();
        let r1 = nums[0]..=nums[1];
        let r2 = nums[2]..=nums[3];
        if r1.contains(&nums[2]) || r1.contains(&nums[3]) || r2.contains(&nums[0]) || r2.contains(&nums[1]) {
            count += 1;
        }
    }
    count
}