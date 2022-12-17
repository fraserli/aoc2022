use std::collections::{HashMap, HashSet, VecDeque};

pub type Id = [char; 2];
pub type Valves = HashMap<Id, Valve>;

const START: Id = ['A', 'A'];

#[derive(Debug)]
pub struct Valve {
    rate: usize,
    connections: Vec<Id>,
}

pub fn process_input(input: &str) -> Valves {
    input
        .lines()
        .map(|l| {
            let mut words = l.split(&[' ', ';', '=', ',']);
            let id: Id = strtoid(words.nth(1).unwrap());
            let rate: usize = words.nth(3).unwrap().parse().unwrap();
            let connections: Vec<Id> = words
                .skip(5)
                .filter(|s| !s.is_empty())
                .map(strtoid)
                .collect();

            (id, Valve { rate, connections })
        })
        .collect()
}

pub fn part1(valves: &Valves) -> usize {
    let ids: Vec<Id> = valves
        .keys()
        .filter(|v| valves[*v].rate > 0)
        .map(|v| *v)
        .collect();

    let mut costs: HashMap<Id, HashMap<Id, usize>> = HashMap::new();
    costs.insert(START, bfs(valves, START, &ids));
    for id in ids.iter() {
        costs.insert(*id, bfs(valves, *id, &ids));
    }

    maxflow(valves, &costs, HashSet::new(), START, 0)
}

pub fn part2(_valves: &Valves) -> usize {
    todo!();
}

fn bfs(valves: &Valves, start: Id, ids: &[Id]) -> HashMap<Id, usize> {
    let mut finished = HashMap::new();
    let mut queue: VecDeque<(Id, usize)> = VecDeque::new();

    queue.push_back((start, 0));
    finished.insert(start, 0);

    while let Some((id, cost)) = queue.pop_front() {
        for conn in valves[&id].connections.iter() {
            if !finished.contains_key(conn) {
                queue.push_back((*conn, cost + 1));
                finished.insert(*conn, cost + 1);
            }
        }
    }

    finished.retain(|id, _| *id != start && ids.contains(id));
    finished.shrink_to_fit();

    finished
}

fn maxflow(
    valves: &Valves,
    ids: &HashMap<Id, HashMap<Id, usize>>,
    mut opened: HashSet<Id>,
    id: Id,
    time: usize,
) -> usize {
    if time > 30 || opened.contains(&id) {
        return 0;
    }

    opened.insert(id);
    let current = valves[&id].rate * (30 - time);

    current
        + ids[&id]
            .iter()
            .map(|(id, cost)| maxflow(valves, ids, opened.clone(), *id, time + cost + 1))
            .max()
            .unwrap_or(0)
}

fn strtoid(s: &str) -> Id {
    s.chars().collect::<Vec<char>>().try_into().unwrap()
}
