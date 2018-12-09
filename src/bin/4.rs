const INPUT: &'static str = include_str!("inputs/4.txt");

const RE_PREFIX: &'static str = r"\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}\]";
const RE_GUARD: &'static str = r"^ Guard #(?P<id>\d+) begins shift$";
const RE_SLEEP: &'static str = r"^ falls asleep$";
const RE_AWAKE: &'static str = r"^ wakes up$";

extern crate regex;
extern crate chrono;
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;

use regex::{Regex, Match};
use chrono::{NaiveDate, NaiveDateTime, Timelike};

lazy_static! {
    static ref re_prefix: Regex = Regex::new(RE_PREFIX).expect("regex should compile");
    static ref re_guard: Regex = Regex::new(RE_GUARD).expect("regex should compile");
    static ref re_sleep: Regex = Regex::new(RE_SLEEP).expect("regex should compile");
    static ref re_awake: Regex = Regex::new(RE_AWAKE).expect("regex should compile");
}

type GuardId = usize;

enum Action {
    BeginShift(GuardId),
    Sleep,
    Awake
}

type Event = (NaiveDateTime, Action);

fn parse_usize(m: Match) -> Option<usize> {
    m.as_str().parse::<usize>().ok()
}

fn parse_line(line: &str) -> Option<Event> {
    let prefix_match = re_prefix.find(line)?;
    let tail = &line[prefix_match.end()..];

    let action = re_sleep.find(tail).map(|_| Action::Sleep)
        .or_else(|| re_awake.find(tail).map(|_| Action::Awake))
        .or_else(|| re_guard.captures(tail)
                 .and_then(|c| parse_usize(c.name("id")?).map(Action::BeginShift)))?;

    let datetime = NaiveDateTime::parse_from_str(prefix_match.as_str(), "[%Y-%m-%d %H:%M]").ok()?;

    Some((datetime, action))
}

fn sorted_events() -> Vec<Event> {
    let mut vec: Vec<Event> = INPUT.lines().map(str::trim).filter(|l| !l.is_empty())
        .map(|l| parse_line(l).expect("should parse"))
        .collect();

    vec.sort_unstable_by(|(e1, _), (e2, _)| e1.cmp(e2));
    vec
}

#[derive(Debug)]
struct Span {
    date: NaiveDate,
    guard_id: GuardId,
    start_min: usize,
    end_min: usize
}

fn compute_spans() -> Vec<Span> {
    let mut guard_id: Option<GuardId> = None;
    let mut asleep: Option<NaiveDateTime> = None;

    let mut ret = Vec::new();

    for (datetime, action) in sorted_events() {
        match action {
            Action::BeginShift(id) => {
                guard_id = Some(id);
                asleep = None;
            },

            Action::Awake => {
                let from = asleep.expect("should be asleep");
                let to = datetime;
                let date = from.date();
                assert!(to.date() == date && from.hour() == 0 && to.hour() == 0);

                ret.push(Span{
                    date,
                    guard_id: guard_id.expect("active guard"),
                    start_min: from.minute() as usize,
                    end_min: to.minute() as usize
                });

                asleep = None;
            },

            Action::Sleep => {
                assert!(asleep.is_none());
                asleep = Some(datetime);
            }
        }
    }

    ret
}

struct Stats {
    total_asleep: u32,
    freqs: [usize; 60]
}

fn compute_stats(spans: &[Span]) -> HashMap<GuardId, Stats> {
    let mut stats = HashMap::new();

    for &Span{ guard_id, start_min, end_min, .. } in spans.iter() {
        let entry = stats.entry(guard_id).or_insert_with(|| Stats{ total_asleep: 0, freqs: [0; 60] });
        for min in start_min..end_min {
            entry.total_asleep += 1;
            entry.freqs[min] += 1;
        }
    }

    stats
}

fn part_one(stats: &HashMap<GuardId, Stats>) {
    let (best_guard_id, _) = stats.iter().max_by_key(|(_, v)| v.total_asleep.clone()).expect("longest sleep exists");
    let (best_minute, _) = stats[best_guard_id].freqs.iter().enumerate().max_by_key(|(_, f)| f.clone()).expect("max freq exists");

    let ans = best_guard_id * best_minute;

    println!("{:?}", ans);
}

fn part_two(stats: &HashMap<GuardId, Stats>) {
    let (best_guard, (best_minute, _)) = stats.iter()
        .map(|(guard_id, Stats{ freqs, .. })| (guard_id, freqs.iter().enumerate().max_by_key(|(_, f)| f.clone()).expect("max freq exists")))
        .max_by_key(|(_, (_, f))| f.clone()).expect("best guard exists");

    let ans = best_guard * best_minute;
    println!("{:?}", ans);
}

fn main() {
    let spans = compute_spans();
    let stats = compute_stats(&spans);
    part_one(&stats);
    part_two(&stats);
}
