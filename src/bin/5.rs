const INPUT: &'static str = include_str!("inputs/5.txt");

use std::collections::HashSet;
use std::iter::FromIterator;

type Seq = Vec<char>;

fn annihilate(w: &[char]) -> bool {
    match w {
        &[c1, c2] =>
            c1.to_ascii_lowercase() == c2.to_ascii_lowercase() &&
            c1.is_lowercase() != c2.is_lowercase(),

        _ => false
    }
}

// O(n^2) - Rust is fast ;)
fn react(mut seq: Seq) -> Seq {
    while let Some((i, _)) = seq.windows(2).enumerate().filter(|(_, w)| annihilate(w)).nth(0) {
        seq.drain(i..(i+2));
    }

    seq
}

fn part_one(seq: Seq) -> Seq {
    let r_seq = react(seq);
    println!("{}", r_seq.len());
    r_seq
}

fn part_two(seq: Seq) {
    let letters: HashSet<char> = HashSet::from_iter(seq.iter().map(char::to_ascii_lowercase));

    let ans = letters
        .iter()
        .map(|&l| react(seq.iter().filter(|c| c.to_ascii_lowercase() != l).cloned().collect()).len())
        .min().expect("answer exists");

    println!("{}", ans);
}

fn main() {
    let seq: Seq = INPUT.trim().chars().collect();
    part_two(part_one(seq));
}
