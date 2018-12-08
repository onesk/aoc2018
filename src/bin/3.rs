const INPUT: &'static str = include_str!("inputs/3.txt");
const N: usize = 1000;

extern crate regex;
#[macro_use] extern crate itertools;

use regex::{Regex, Match};

#[derive(Debug)]
struct Rect {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize
}

impl Rect {
    fn pixels(&self) -> impl Iterator<Item=(usize, usize)> {
        iproduct!(self.x..(self.x+self.w), self.y..(self.y+self.h))
    }
}

fn parse_usize_match(om: Option<Match>) -> Option<usize> {
    om.and_then(|m| m.as_str().parse::<usize>().ok())
}

fn rects() -> impl Iterator<Item=Rect> {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")
        .expect("constant regex is always correct, qed.");

    INPUT.lines()
        .filter_map(move |line| re.captures(line))
        .map(|captures| captures.iter().filter_map(parse_usize_match).collect::<Vec<_>>())
        .filter_map(|uss| match uss.as_slice() {
            &[id, x, y, w, h] => Some(Rect{ id, x, y, w, h}),
            _ => None
        })
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum CellRect {
    Free,
    Single,
    Multiple
}

impl CellRect {
    fn fill(&self) -> CellRect {
        match self {
            CellRect::Free => CellRect::Single,
            CellRect::Single => CellRect::Multiple,
            CellRect::Multiple => CellRect::Multiple
        }
    }
}

fn part_one() {
    let mut grid = [[0usize; N]; N];

    rects().map(|r| r.pixels()).flatten().for_each(|(i, j)| {
        grid[i][j] += 1;
    });

    let ans = grid.iter().flat_map(|s| s.iter()).filter(|&&c| c > 1usize).count();
    println!("{}", ans);
}

fn part_two() {
    let mut grid = [[CellRect::Free; N]; N];

    for r in rects() {
        for (i, j) in r.pixels() {
            grid[i][j] = grid[i][j].fill();
        }
    }

    for r in rects() {
        if r.pixels().all(|(i, j)| grid[i][j] != CellRect::Multiple) {
            println!("{}", r.id);
        }
    }
}

fn main() {
    part_one();
    part_two();
}
