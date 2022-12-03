use std::collections::HashSet;

fn main() -> Result<(), ()> {
    let score: u32 = include_str!("day3.input")
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .flat_map(|sacks| {
            sacks
                .iter()
                .map(|sack| sack.chars().collect::<HashSet<_>>())
                .reduce(|accum, current| {
                    accum
                        .intersection(&current)
                        .map(|x| *x)
                        .collect::<HashSet<_>>()
                })
                .unwrap()
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
