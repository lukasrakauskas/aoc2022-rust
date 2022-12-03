use std::collections::HashSet;

fn main() -> Result<(), ()> {
    let score: u32 = include_str!("day3.input")
        .lines()
        .flat_map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let first = first.chars().collect::<HashSet<char>>();
            let second = second.chars().collect::<HashSet<char>>();

            first.intersection(&second).map(|x| *x).collect::<Vec<_>>()
        })
        .map(|it| match it {
            'a'..='z' => it as u32 - 'a' as u32 + 1,
            'A'..='Z' => it as u32 - 'A' as u32 + 27,
            _ => 0,
        })
        .sum();

    println!("Score: {:?}", score);

    Ok(())
}
