const INPUT: &'static str = include_str!("inputs/7.txt");
const WORKERS: usize = 5;

extern crate regex;
#[macro_use] extern crate lazy_static;

use std::collections::{HashSet, HashMap};

use regex::Regex;

lazy_static! {
    static ref line_re: Regex = Regex::new("Step (.) must be finished before step (.) can begin.")
        .expect("regex should compile");
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Letter(u8);

impl Letter {
    fn from_string(symbol: &str) -> Option<Letter> {
        if symbol.len() != 1 {
            return None;
        }

        match symbol.chars().next() {
            Some(ch @ 'A'...'Z') => Some(Letter(ch as u8 - 'A' as u8)),
            _ => None
        }
    }

    fn seconds(&self) -> usize {
        self.0 as usize + 61
    }

    fn char(&self) -> char {
        (self.0 + 'A' as u8) as char
    }
}

#[derive(Debug)]
struct Edge {
    from: Letter,
    to: Letter
}

fn parse_line(line: &str) -> Option<Edge> {
    let captures = line_re.captures(line)?;
    Some(Edge {
        from: Letter::from_string(captures.get(1)?.as_str())?,
        to: Letter::from_string(captures.get(2)?.as_str())?
    })
}

fn edges() -> Vec<Edge> {
    INPUT.lines()
        .map(|line| parse_line(line))
        .collect::<Option<Vec<Edge>>>()
        .expect("everything should parse")
}

fn get_sources(edges: &[Edge]) -> HashSet<Letter> {
    let unique: HashSet<_> = edges.iter()
        .flat_map(|Edge{ ref from, ref to }| vec![from, to])
        .cloned().collect();

    let sources: HashSet<_> = unique.iter()
        .filter(|&v| edges.iter().all(|Edge{ to, .. }| to != v))
        .cloned().collect();

    sources
}

fn get_incoming(edges: &[Edge]) -> HashMap<Letter, usize> {
    let mut incoming: HashMap<_, usize> = HashMap::new();
    for Edge{ to, .. } in edges {
        *incoming.entry(to.clone()).or_default() += 1;
    };

    incoming
}

fn join_letters(letters: &[Letter]) -> String {
    letters.iter().map(Letter::char).collect::<String>()
}

fn part_one(edges: &[Edge]) {
    let sources = get_sources(edges);
    let mut incoming = get_incoming(edges);

    let mut frontier = sources;
    let mut ans = Vec::new();

    while let Some(cur) = frontier.iter().min().cloned() {
        frontier.remove(&cur);
        ans.push(cur);

        for Edge{ to, ..} in edges.iter().filter(|Edge{ from, .. }| *from == cur) {
            let cnt = incoming.get_mut(to).expect("should exist by definition, qed.");
            *cnt -= 1;
            if *cnt == 0 {
                frontier.insert(to.clone());
            }
        }
    }

    println!("{}", join_letters(&ans))
}

fn part_two(edges: &[Edge]) {
    let sources = get_sources(edges);
    let mut incoming = get_incoming(edges);

    let mut queue = HashSet::new();
    let mut frontier = sources;

    let mut total_time: usize = 0;

    loop {
        while queue.len() < WORKERS {
            if let Some(cur) = frontier.iter().min().cloned() {
                queue.insert((cur, total_time + cur.seconds()));
                frontier.remove(&cur);
            } else {
                break;
            }
        }

        let nearest = if let Some(nearest) = queue.iter().min_by_key(|(_, t)| t).cloned() {
            nearest
        } else {
            break;
        };

        queue.remove(&nearest);

        let (min_letter, nearest_time) = nearest;
        total_time = nearest_time;

        for Edge{ to, ..} in edges.iter().filter(|Edge{ from, .. }| *from == min_letter) {
            let cnt = incoming.get_mut(to).expect("should exist by definition, qed.");
            *cnt -= 1;
            if *cnt == 0 {
                frontier.insert(to.clone());
            }
        }
    }

    println!("{}", total_time);
}

fn main() {
    let edges = edges();
    part_one(&edges);
    part_two(&edges);
}
