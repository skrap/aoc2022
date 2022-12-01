fn main() {
    let input = include_str!("../input");
    let mut elves: Vec<_> = input.split("\n\n").map(|elf| {
        elf.split_whitespace().map(|n| n.parse::<i32>().unwrap()).sum::<i32>()
    }).collect();
    elves.sort();
    elves.reverse();
    println!("part 1: {}", elves[0]);
    println!("part 2: {}", elves[0]+elves[1]+elves[2]);
}
