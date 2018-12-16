const INPUT: &'static str = include_str!("inputs/6.txt");

extern crate ndarray;
#[macro_use] extern crate itertools;

use std::collections::VecDeque;
use std::iter::FromIterator;

use itertools::Itertools;
use ndarray::{Array2, indices_of};

const STEPS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const DIST_LIMIT: usize = 10000;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn adj<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        STEPS.iter().map(move |(dx, dy)| Point{ x: self.x + dx, y: self.y + dy })
    }

    fn dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct Bounds {
    min: i32,
    max: i32
}

impl Bounds {
    fn size(&self) -> usize {
        (self.max - self.min + 1) as usize
    }

    fn rel(&self, x: i32) -> Option<usize> {
        if self.min <= x && x <= self.max { Some((x - self.min) as usize) } else { None }
    }

    fn expand(&self, span: usize) -> Bounds {
        Bounds{ min: self.min - span as i32, max: self.max + span as i32 }
    }

    fn range(&self) -> impl Iterator<Item=i32> + Clone {
        self.min..(self.max+1)
    }
}

#[derive(Debug)]
struct AABB {
    x: Bounds,
    y: Bounds
}

impl AABB {
    fn size(&self) -> (usize, usize) {
        (self.x.size(), self.y.size())
    }

    fn rel(&self, p: Point) -> Option<(usize, usize)> {
        Some((self.x.rel(p.x)?, self.y.rel(p.y)?))
    }

    fn is_boundary(&self, (x, y): (usize, usize)) -> bool {
        x == 0 || x+1 == self.x.size() || y == 0 || y+1 == self.y.size()
    }

    fn expand(&self, span: usize) -> AABB {
        AABB{ x: self.x.expand(span), y: self.y.expand(span) }
    }

    fn domain(&self) -> impl Iterator<Item=Point> {
        iproduct!(self.x.range(), self.y.range()).map(|(x, y)| Point{ x, y })
    }
}

fn parse_line(line: &str) -> Option<Point> {
    line.split(", ")
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()
        .ok()
        .and_then(|vec| match vec.as_slice() {
            &[x, y] => Some(Point{ x, y }),
            _ => None
        })
}

fn parse_points() -> Vec<Point> {
    INPUT.lines().map(str::trim).filter(|l| !l.is_empty())
        .map(|l| parse_line(l))
        .collect::<Option<Vec<Point>>>()
        .expect("everything should parse")
}

fn bounds_by_key<F>(points: &[Point], f: F) -> Bounds where F: Fn(&Point) -> i32 {
    points
        .iter()
        .map(f)
        .minmax()
        .into_option()
        .map(|(min, max)| Bounds{ min, max })
        .expect("more than one")
}

fn compute_aabb(points: &[Point]) -> AABB {
    AABB {
        x: bounds_by_key(points, |p| p.x),
        y: bounds_by_key(points, |p| p.y)
    }
}

#[derive(Debug, Clone, Copy)]
enum SlotMark {
    Empty,
    Closest{ depth: usize, id: usize },
    Tie
}

fn part_one(points: &[Point]) {
    let aabb = compute_aabb(points);

    let mut slots = Array2::from_elem(aabb.size(), SlotMark::Empty);

    for (id, &p) in points.iter().enumerate() {
        slots[aabb.rel(p).expect("all within aabb, qed")] = SlotMark::Closest{ depth: 0, id };
    }

    let mut queue = VecDeque::from_iter(points.iter().cloned().enumerate());

    while let Some((cur_id, cur_p)) = queue.pop_front() {
        let ix = aabb.rel(cur_p).expect("should be within aabb");

        let cur_depth = match slots[ix] {
            SlotMark::Tie => { continue; },
            SlotMark::Closest{ depth, id } if id == cur_id => depth,
            _ => unreachable!()
        };

        for (adj_p, adj_ix) in cur_p.adj().filter_map(|p| aabb.rel(p).map(|ix| (p, ix))) {
            match slots[adj_ix] {
                SlotMark::Empty => {
                    slots[adj_ix] = SlotMark::Closest{ depth: cur_depth + 1, id: cur_id };
                    queue.push_back((cur_id, adj_p));
                },

                SlotMark::Closest{ depth, id } if id != cur_id && depth == cur_depth + 1 => {
                    slots[adj_ix] = SlotMark::Tie;
                },

                _ => {}
            }
        }
    }

    let mut boundary = vec![false; points.len()];
    let mut counts = vec![0usize; points.len()];

    for ix in indices_of(&slots) {
        if let SlotMark::Closest{ id, .. } = slots[ix] {
            counts[id] += 1;
            if aabb.is_boundary(ix) {
                boundary[id] = true;
            }
        }
    }

    let max_area = counts.iter().zip(boundary.iter())
        .filter_map(|(cnt, is_boundary)| if !is_boundary { Some(cnt) } else { None })
        .max()
        .expect("there are finite regions by definition");

    println!("{:?}", max_area);
}

fn part_two(points: &[Point]) {
    let aabb = compute_aabb(points);
    let wide_aabb = aabb.expand(DIST_LIMIT / points.len());

    let region_size = wide_aabb.domain().filter_map(|domain_p| {
        let below_threshold = points.iter()
            .scan(0, |state, &p| {
                *state += p.dist(&domain_p);
                Some(*state)
            })
            .take_while(|&total_dist| total_dist < DIST_LIMIT as i32)
            .count();

        if below_threshold == points.len() { Some(domain_p) } else { None }
    }).count();

    println!("{:?}", region_size);
}

fn main() {
    let points: Vec<Point> = parse_points();
    part_one(&points);
    part_two(&points);
}
