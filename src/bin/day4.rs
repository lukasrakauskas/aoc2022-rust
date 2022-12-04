use std::{num::ParseIntError, str::FromStr};

fn main() -> Result<(), ()> {
    println!("Problem one: {:?}", solve(fully_contains));
    println!("Problem two: {:?}", solve(partially_contains));

    Ok(())
}

fn solve(contains: fn(assignments: (Assignment, Assignment)) -> bool) -> usize {
    include_str!("day4.input")
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(first, second)| {
                    (
                        first.parse::<Assignment>().unwrap(),
                        second.parse::<Assignment>().unwrap(),
                    )
                })
                .unwrap()
        })
        .map(contains)
        .filter(|x| *x)
        .count()
}

struct Assignment {
    from: u32,
    to: u32,
}

impl FromStr for Assignment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once('-').unwrap();
        let from = from.parse::<u32>()?;
        let to = to.parse::<u32>()?;

        Ok(Assignment { from, to })
    }
}

fn inside(a: &Assignment, b: &Assignment) -> bool {
    a.from <= b.from && a.to >= b.to
}

fn partly_inside(a: &Assignment, b: &Assignment) -> bool {
    b.from <= a.to && b.from >= a.from
}

fn fully_contains(assignments: (Assignment, Assignment)) -> bool {
    let (a, b) = assignments;
    return inside(&a, &b) || inside(&b, &a);
}

fn partially_contains(assignments: (Assignment, Assignment)) -> bool {
    let (a, b) = assignments;
    return partly_inside(&a, &b) || partly_inside(&b, &a);
}
