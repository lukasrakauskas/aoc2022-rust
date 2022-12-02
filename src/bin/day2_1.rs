use std::collections::HashMap;

fn main() -> Result<(), ()> {
    let score_map = HashMap::from([
        ("A Z", 0),
        ("B X", 0),
        ("C Y", 0),
        ("A X", 3),
        ("B Y", 3),
        ("C Z", 3),
        ("A Y", 6),
        ("B Z", 6),
        ("C X", 6),
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    let input = include_str!("day2.sample");
    let lines = input.lines();

    let mut score = 0;

    for line in lines {
        let x = line.split(" ").skip(1).take(1).last().unwrap_or("");

        let hand_score = score_map.get(x).unwrap_or(&0);
        let versus_score = score_map.get(line).unwrap_or(&0);

        score += hand_score;
        score += versus_score;
    }

    println!("Score: {:?}", score);
    Ok(())
}
