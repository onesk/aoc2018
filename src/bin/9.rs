use std::iter::{once, FromIterator};
use std::collections::VecDeque;

const N_PLAYERS: usize = 463;
const N_MARBLES: usize = 71787;

struct Circle(VecDeque<usize>);

impl Circle {
    fn new(elem: usize) -> Self {
        Circle(VecDeque::from_iter(once(elem)))
    }

    fn rotate_cw(&mut self) {
        let front = self.0.pop_front().expect("circle nonempty");
        self.0.push_back(front);
    }

    fn rotate_ccw(&mut self) {
        let back = self.0.pop_back().expect("circle nonempty");
        self.0.push_front(back);
    }

    fn insert_step(&mut self, elem: usize) {
        self.rotate_cw();
        self.0.push_back(elem);
    }

    fn strange_step(&mut self) -> usize {
        (0..7).for_each(|_| self.rotate_ccw());
        let evicted = self.0.pop_back().expect("circle nonempty");
        self.rotate_cw();
        evicted
    }
}

fn highscore(n_players: usize, last_marble: usize) -> usize {
    let mut scores = vec![0usize; n_players];
    let mut circle = Circle::new(0);

    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            let rem = circle.strange_step();
            scores[marble % n_players] += marble + rem;

        } else {
            circle.insert_step(marble);
        }
    }

    scores.iter().max().cloned().expect("at least one player")
}

fn part_one() {
    println!("{}", highscore(N_PLAYERS, N_MARBLES));
}

fn part_two() {
    println!("{}", highscore(N_PLAYERS, N_MARBLES * 100));
}

fn main() {
    part_one();
    part_two();
}

#[test]
fn examples() {
    assert_eq!(highscore(10, 1618), 8317);
    assert_eq!(highscore(13, 7999), 146373);
    assert_eq!(highscore(17, 1104), 2764);
    assert_eq!(highscore(21, 6111), 54718);
    assert_eq!(highscore(30, 5807), 37305);
}
