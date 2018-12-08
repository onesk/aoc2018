use std::collections::HashSet;

const INPUT: &'static str = include_str!("inputs/1.txt");

fn numbers() -> impl Iterator<Item=i32> + Clone {
    INPUT.lines().map(|s| s.parse::<i32>().expect("integer"))
}

fn part_one() {
    let ans = numbers().fold(0, |s, n| s+n);
    println!("{}", ans);
}

fn part_two() {
    let mut repeating_sums = numbers().cycle()
        .scan(0i32, |s, n| { *s += n; Some(*s) })
        .scan(HashSet::new(), |seen, sum| Some(seen.replace(sum)))
        .filter_map(|option_sum| option_sum);

    println!("{}", repeating_sums.nth(0).expect("challenge has a solution, qed."));
}

fn main() {
    part_one();
    part_two();
}
