use std::collections::HashMap;

type Tree = Vec<Node>;

pub struct Node {
    pub parent: Option<usize>,
    pub subdirs: HashMap<String, usize>,
    pub files: HashMap<String, usize>,
}

impl Node {
    pub fn new(parent: usize) -> Self {
        Self {
            parent: Some(parent),
            subdirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

pub fn process_input(input: &str) -> Tree {
    let mut tree = Tree::new();
    tree.push(Node::new(0));

    let mut current_node: usize = 0;

    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        let mut words = line.split(' ');
        assert_eq!(words.next(), Some("$"));

        match words.next().unwrap() {
            "ls" => {
                while lines.peek().is_some() && !lines.peek().unwrap().starts_with('$') {
                    let mut words = lines.next().unwrap().split(' ');
                    let word1 = words.next().unwrap();
                    let word2 = words.next().unwrap();
                    match word1 {
                        "dir" => {
                            if !tree[current_node].subdirs.contains_key(word2) {
                                let id = tree.len();
                                let subdir = Node::new(current_node);
                                tree.push(subdir);
                                tree[current_node].subdirs.insert(word2.to_owned(), id);
                            }
                        }
                        _ => {
                            let size: usize = word1.parse().unwrap();
                            let name = word2.to_owned();
                            tree[current_node].files.insert(name, size);
                        }
                    }
                }
            }
            "cd" => {
                let dir = words.next().unwrap();
                match dir {
                    ".." => {
                        current_node = tree[current_node].parent.unwrap();
                    }
                    "/" => {
                        current_node = 0;
                    }
                    _ => {
                        if tree[current_node].subdirs.get(dir).is_some() {
                            current_node = tree[current_node].subdirs[dir];
                        } else {
                            panic!();
                        }
                    }
                }
            }
            _ => panic!(),
        }
    }

    tree
}

pub fn part1(data: &Tree) -> usize {
    let mut sizes = Vec::new();
    all_sizes(data, 0, &mut sizes);
    sizes.iter().filter(|s| **s <= 100000).sum()
}

pub fn part2(data: &Tree) -> usize {
    let mut sizes = Vec::new();
    let unused = 70000000 - all_sizes(data, 0, &mut sizes);
    let diff = 30000000 - unused;

    sizes.sort();
    *sizes.iter().find(|n| **n >= diff).unwrap()
}

fn all_sizes(tree: &Tree, node: usize, sizes: &mut Vec<usize>) -> usize {
    let filesizes: usize = tree[node].files.values().sum();
    let mut dirsizes = 0;
    for subdir in tree[node].subdirs.values() {
        dirsizes += all_sizes(tree, *subdir, sizes);
    }

    let totalsize = filesizes + dirsizes;
    sizes.push(totalsize);

    totalsize
}
