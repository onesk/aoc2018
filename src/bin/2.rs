const INPUT: &'static str = include_str!("inputs/2.txt");

use std::collections::HashMap;

fn words() -> impl Iterator<Item=&'static str> + Clone {
    INPUT.lines().map(str::trim)
}

type LetterCounts = HashMap<char, usize>;

fn letter_counts(s: &str) -> LetterCounts {
    let mut counts = HashMap::with_capacity(26);
    for c in s.chars() {
        let count = counts.entry(c).or_default();
        *count += 1;
    }

    counts
}

fn part_one() {
    let counts = words().map(letter_counts);
    let s2 = counts.clone().filter(|lc| lc.values().any(|c| *c == 2)).count();
    let s3 = counts.clone().filter(|lc| lc.values().any(|c| *c == 3)).count();
    println!("{}", s2 * s3)
}

fn part_two() {
    let words_vec: Vec<_> = words().collect();

    for (i, w1) in words_vec.iter().enumerate() {
        for w2 in words_vec[i+1..].iter() {
            assert!(w1.len() == w2.len());
            let same_letters: String = w1.chars().zip(w2.chars())
                .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
                .collect();

            if same_letters.len() + 1 == w1.len() {
                println!("{}", same_letters);
            }
        }
    }
}

fn main() {
    part_one();
    part_two();
}
