use std::cmp::Ordering;
use std::collections::VecDeque;

pub type Tree = Vec<Node>;

#[derive(Clone, PartialEq, Eq)]
pub enum Node {
    List(Vec<usize>),
    Number(usize),
}

pub fn process_input(input: &str) -> Vec<(Tree, Tree)> {
    let mut pairs = Vec::new();

    for group in input.split("\n\n") {
        let mut lines = group.lines();
        let t1 = parse(lines.next().unwrap());
        let t2 = parse(lines.next().unwrap());
        pairs.push((t1, t2));
    }

    pairs
}

pub fn part1(data: &[(Tree, Tree)]) -> usize {
    data.iter()
        .enumerate()
        .map(|(i, (t1, t2))| {
            if compare(t1, t2, 0, 0) != Ordering::Greater {
                i + 1
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(data: &[(Tree, Tree)]) -> usize {
    let mut packets = Vec::with_capacity(data.len() * 2);
    for (p1, p2) in data {
        packets.push(p1);
        packets.push(p2);
    }
    let k1 = parse("[[2]]");
    let k2 = parse("[[6]]");
    packets.push(&k1);
    packets.push(&k2);

    packets.sort_by(|p1, p2| compare(p1, p2, 0, 0));

    let i1 = packets.iter().position(|p| **p == k1).unwrap();
    let i2 = packets.iter().position(|p| **p == k2).unwrap();

    (i1 + 1) * (i2 + 1)
}

fn compare(t1: &Tree, t2: &Tree, n1: usize, n2: usize) -> Ordering {
    use Node::*;

    match (&t1[n1], &t2[n2]) {
        (Number(n1), Number(n2)) => n1.cmp(n2),
        (List(l1), List(l2)) => {
            for (&x1, &x2) in l1.iter().zip(l2.iter()) {
                let c = compare(t1, t2, x1, x2);
                if c != Ordering::Equal {
                    return c;
                }
            }

            l1.len().cmp(&l2.len())
        }
        (Number(n), List(_)) => {
            let mut t = t1.to_vec(); // HACK
            let idx = t.len();
            t.push(Node::Number(*n));
            t[n1] = Node::List(Vec::from([idx]));
            compare(&t, t2, n1, n2)
        }
        (List(_), Number(n)) => {
            let mut t = t2.to_vec();
            let idx = t.len();
            t.push(Node::Number(*n));
            t[n2] = Node::List(Vec::from([idx]));
            compare(t1, &t, n1, n2)
        }
    }
}

#[derive(PartialEq, Eq)]
enum Token {
    Open,
    Close,
    Number(usize),
}

fn parse(source: &str) -> Tree {
    let mut tree = Tree::new();
    tree.push(Node::List(Vec::new()));

    let mut tokens = VecDeque::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '[' => tokens.push_back(Token::Open),
            ']' => tokens.push_back(Token::Close),
            ',' => {}
            _ => {
                let s: String = if let Some(d) = chars.peek() && d.is_numeric() {
                    [c, chars.next().unwrap()].iter().collect()
                } else {
                    [c].iter().collect()
                };
                tokens.push_back(Token::Number(s.parse().unwrap()));
            }
        }
    }

    parse_list(&mut tokens, &mut tree, 0);

    tree
}

/* Implemented with recursive descent */
fn parse_list(tokens: &mut VecDeque<Token>, tree: &mut Tree, n: usize) {
    while let Some(t) = tokens.pop_front() {
        match t {
            Token::Open => {
                let idx = tree.len();
                tree.push(Node::List(Vec::new()));
                if let Node::List(node) = tree.get_mut(n).unwrap() {
                    node.push(idx);
                } else {
                    panic!();
                }
                parse_list(tokens, tree, idx);
            }
            Token::Number(num) => {
                let idx = tree.len();
                tree.push(Node::Number(num));
                if let Node::List(node) = tree.get_mut(n).unwrap() {
                    node.push(idx);
                } else {
                    panic!();
                }
            }
            Token::Close => break,
        }
    }
}

/*fn print_packet(tree: &Tree) {
    print_node(tree, 1);
    println!();
}

fn print_node(tree: &Tree, n: usize) {
    match &tree[n] {
        Node::List(l) => {
            print!("[");
            for node in l {
                print_node(tree, *node);
            }
            print!("]");
        },
        Node::Number(n) => print!(" {n} "),
    }
}*/
