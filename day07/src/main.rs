use std::collections::HashMap;

fn main() {
    let input = include_str!("../input").trim();
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> usize {
    let mut result = 0;
    let tree = parse(input);
    for (_dirpath, size) in &tree {
        if *size <= 100000 {
            result += size;
        }
    }
    result
}

fn part2(input: &str) -> usize {
    let tree = parse(input);
    let needed = 30000000 - (70000000 - tree["/"]);
    let mut candidate_sizes = vec![];
    for (_dirpath, size) in &tree {
        if *size >= needed {
            candidate_sizes.push(size);
        }
    }
    candidate_sizes.sort();
    *candidate_sizes[0]
}

type Tree = HashMap<String,usize>; // dirs only
fn add_dir(tree: &mut Tree, path: &[&str], name: &str) {
    let fullpath = if path.is_empty() {
        format!("/{}", name)
    } else {
        format!("/{}/{}", path.join("/"), name)
    };
    tree.insert(fullpath, 0);
}

fn add_file(tree: &mut Tree, path: &[&str], name: &str, size: usize) {
    let mut path = path.clone();
    loop {
        let fullpath = if path.is_empty() { "/".to_string() } else { format!("/{}", path.join("/")) };
        *tree.entry(fullpath).or_default() += size;
        if path.is_empty() {
            break;
        }
        path = path.split_last().unwrap().1;
    }
}

fn parse(input: &str) -> Tree {
    let mut tree = Tree::new();
    let mut path = vec![];

    for line in input.lines() {
        if line == "$ cd /" {
            path = vec![];
        } else if line == "$ cd .." {
            path.pop().unwrap();
        } else if line.starts_with("$ cd ") {
            let subdir = line.rsplit_once(" ").unwrap().1;
            path.push(subdir);
        } else if line == "$ ls" {
            // do nothing
        } else if line.starts_with("dir") {
            let name = line.rsplit_once(" ").unwrap().1;
            add_dir(&mut tree, &path, name);
        } else {
            // file
            let (size, name) = line.split_once(" ").unwrap();
            add_file(&mut tree, &path, name, size.parse().unwrap());
        }
    }

    tree
}

#[test]
fn test_part1and2() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    assert_eq!(part1(input), 95437);
    assert_eq!(part2(input), 24933642);
}

